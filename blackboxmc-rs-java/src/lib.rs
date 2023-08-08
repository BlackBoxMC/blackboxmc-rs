#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct JavaArrayList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaArrayList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaArrayList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaArrayList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaArrayList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaArrayList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/ArrayList")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn ensure_capacity(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ensureCapacity",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn trim_to_size(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "trimToSize", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaArrayList<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaTreeSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTreeSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTreeSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaTreeSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaTreeSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaTreeSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/TreeSet")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/SortedSet;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub unsafe fn new_with_comparator(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/TreeSet")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub fn poll_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn descending_iterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn sub_set_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<bool>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
        arg3: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let val_3 = arg2.unwrap();
        // 1
        let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subSet",
            "(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn head_set_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "headSet",
            "(Ljava/lang/Object;Z)Ljava/util/NavigableSet;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn tail_set_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tailSet",
            "(Ljava/lang/Object;Z)Ljava/util/NavigableSet;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn descending_set(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingSet",
            "()Ljava/util/NavigableSet;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn ceiling(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceiling",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn higher(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higher",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn floor(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floor",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "last", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "first", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn lower(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lower",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn comparator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "comparator",
            "()Ljava/util/Comparator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
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
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaTreeSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaVector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaVector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaVector<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaVector from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaVector")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaVector object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/Vector")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Collection;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Vector")?;
        let res = jni.new_object(
            cls,
            "(II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn copy_into(
        &mut self,
        arg0: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyInto",
            "(Ljava/lang/Object;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_element_at(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeElementAt",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_element(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeElement",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn insert_element_at(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "insertElementAt",
            "(Ljava/lang/Object;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_all_elements(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "removeAllElements", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn first_element(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "firstElement",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last_element(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastElement",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn set_element_at(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setElementAt",
            "(Ljava/lang/Object;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_element(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addElement",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_size(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSize",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_with_int(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn index_of_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn elements(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "elements",
            "()Ljava/util/Enumeration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn capacity(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "capacity", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn ensure_capacity(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ensureCapacity",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn trim_to_size(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "trimToSize", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn element_at(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "elementAt",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaVector<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaRandomAccess. Needed for returning it from Java.
pub struct JavaRandomAccess<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomAccess<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaRandomAccess from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomAccess")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandomAccess object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomAccess<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaAbstractMapSimpleImmutableEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMapSimpleImmutableEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractMapSimpleImmutableEntry<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractMapSimpleImmutableEntry from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractMapSimpleImmutableEntry")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractMapSimpleImmutableEntry object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new_with_mapentry(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let cls = &jni.find_class("Javajava/util/AbstractMap$SimpleImmutableEntry")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn equals(
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
    pub fn value(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getValue", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKey", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn set_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractMapSimpleImmutableEntry<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaMap. Needed for returning it from Java.
pub struct JavaMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn copy_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Map")?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            "(Ljava/util/Map;)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
        arg3: std::option::Option<jni::objects::JObject<'mc>>,
        arg4: std::option::Option<jni::objects::JObject<'mc>>,
        arg5: std::option::Option<jni::objects::JObject<'mc>>,
        arg6: std::option::Option<jni::objects::JObject<'mc>>,
        arg7: std::option::Option<jni::objects::JObject<'mc>>,
        arg8: std::option::Option<jni::objects::JObject<'mc>>,
        arg9: std::option::Option<jni::objects::JObject<'mc>>,
        arg10: std::option::Option<jni::objects::JObject<'mc>>,
        arg11: std::option::Option<jni::objects::JObject<'mc>>,
        arg12: std::option::Option<jni::objects::JObject<'mc>>,
        arg13: std::option::Option<jni::objects::JObject<'mc>>,
        arg14: std::option::Option<jni::objects::JObject<'mc>>,
        arg15: std::option::Option<jni::objects::JObject<'mc>>,
        arg16: std::option::Option<jni::objects::JObject<'mc>>,
        arg17: std::option::Option<jni::objects::JObject<'mc>>,
        arg18: std::option::Option<jni::objects::JObject<'mc>>,
        arg19: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let val_4 = arg3.unwrap();
        let val_5 = arg4.unwrap();
        let val_6 = arg5.unwrap();
        let val_7 = arg6.unwrap();
        let val_8 = arg7.unwrap();
        let val_9 = arg8.unwrap();
        let val_10 = arg9.unwrap();
        let val_11 = arg10.unwrap();
        let val_12 = arg11.unwrap();
        let val_13 = arg12.unwrap();
        let val_14 = arg13.unwrap();
        let val_15 = arg14.unwrap();
        let val_16 = arg15.unwrap();
        let val_17 = arg16.unwrap();
        let val_18 = arg17.unwrap();
        let val_19 = arg18.unwrap();
        let val_20 = arg19.unwrap();
        let cls = &jni.find_class("Javajava/util/Map")?;
        let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10),jni::objects::JValueGen::from(&val_11),jni::objects::JValueGen::from(&val_12),jni::objects::JValueGen::from(&val_13),jni::objects::JValueGen::from(&val_14),jni::objects::JValueGen::from(&val_15),jni::objects::JValueGen::from(&val_16),jni::objects::JValueGen::from(&val_17),jni::objects::JValueGen::from(&val_18),jni::objects::JValueGen::from(&val_19),jni::objects::JValueGen::from(&val_20)])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn entry(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let cls = &jni.find_class("Javajava/util/Map$Entry")?;
        let res = jni.call_static_method(
            cls,
            "entry",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of_entries(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Map")?;
        let res = jni.call_static_method(
            cls,
            "ofEntries",
            "(Ljava/util/Map$Entry;)Ljava/util/Map;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaWeakHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaWeakHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaWeakHashMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaWeakHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaWeakHashMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaWeakHashMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/WeakHashMap")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/WeakHashMap")?;
        let res = jni.new_object(
            cls,
            "(IF)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaWeakHashMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaSet. Needed for returning it from Java.
pub struct JavaSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn copy_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Set")?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            "(Ljava/util/Collection;)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Set")?;
        let res = jni.call_static_method(cls, "of", "(Ljava/lang/Object;)Ljava/util/Set;", &[])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of_with_object(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
        arg3: jni::objects::JObject<'mc>,
        arg4: jni::objects::JObject<'mc>,
        arg5: jni::objects::JObject<'mc>,
        arg6: jni::objects::JObject<'mc>,
        arg7: jni::objects::JObject<'mc>,
        arg8: jni::objects::JObject<'mc>,
        arg9: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let val_4 = arg3;
        let val_5 = arg4;
        let val_6 = arg5;
        let val_7 = arg6;
        let val_8 = arg7;
        let val_9 = arg8;
        let val_10 = arg9.unwrap();
        let cls = &jni.find_class("Javajava/util/Set")?;
        let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10)])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaBase64<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaBase64 from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaBase64")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBase64 object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn encoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Base64$Encoder")?;
        let res = jni.call_static_method(cls, "getEncoder", "()Ljava/util/Base64$Encoder;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn url_encoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Base64$Encoder")?;
        let res =
            jni.call_static_method(cls, "getUrlEncoder", "()Ljava/util/Base64$Encoder;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn get_mime_encoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: Vec<i8>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = &jni.find_class("Javajava/util/Base64$Encoder")?;
        let res = jni.call_static_method(
            cls,
            "getMimeEncoder",
            "(IB)Ljava/util/Base64$Encoder;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn decoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Base64$Decoder")?;
        let res = jni.call_static_method(cls, "getDecoder", "()Ljava/util/Base64$Decoder;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn url_decoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Base64$Decoder")?;
        let res =
            jni.call_static_method(cls, "getUrlDecoder", "()Ljava/util/Base64$Decoder;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn mime_decoder(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Base64$Decoder")?;
        let res =
            jni.call_static_method(cls, "getMimeDecoder", "()Ljava/util/Base64$Decoder;", &[])?;
        Ok(res.l().unwrap())
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
    pub unsafe fn equals(
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
pub struct JavaAbstractCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractCollection<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractCollection from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractCollection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractCollection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub unsafe fn equals(
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractCollection<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractQueue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractQueue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractQueue<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractQueue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractQueue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractQueue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn element(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "element", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub unsafe fn equals(
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn offer(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn poll(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "poll", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peek", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractQueue<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractMapSimpleEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractMapSimpleEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractMapSimpleEntry<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractMapSimpleEntry from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractMapSimpleEntry")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractMapSimpleEntry object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new_with_mapentry(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let cls = &jni.find_class("Javajava/util/AbstractMap$SimpleEntry")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn equals(
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
    pub fn value(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getValue", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKey", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn set_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractMapSimpleEntry<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaObserver. Needed for returning it from Java.
pub struct JavaObserver<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaObserver<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaObserver from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaObserver")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaObserver object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn update(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(Ljava/util/Observable;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaObserver<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaEventListener. Needed for returning it from Java.
pub struct JavaEventListener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaEventListener<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaEventListener from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaEventListener")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaEventListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaEventListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaQueue. Needed for returning it from Java.
pub struct JavaQueue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaQueue<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaQueue from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaQueue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaQueue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn offer(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn poll(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "poll", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peek", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn element(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "element", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaQueue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaQueue<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaEnumeration. Needed for returning it from Java.
pub struct JavaEnumeration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaEnumeration<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaEnumeration from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaEnumeration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaEnumeration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn as_iterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "asIterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn has_more_elements(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMoreElements", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next_element(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextElement",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaEnumeration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaHashtable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaHashtable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaHashtable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashtable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaHashtable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaHashtable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Hashtable")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Hashtable")?;
        let res = jni.new_object(
            cls,
            "(IF)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn elements(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "elements",
            "()Ljava/util/Enumeration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn keys(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "keys",
            "()Ljava/util/Enumeration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaHashtable<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractSet from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn equals(
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
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaOptionalInt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalInt<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalInt from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaOptionalInt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaOptionalInt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn as_int(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsInt", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn if_present_or_else(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresentOrElse",
            "(Ljava/util/function/IntConsumer;Ljava/lang/Runnable;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn or_else_get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            "(Ljava/util/function/IntSupplier;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            "(Ljava/util/function/IntConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn equals(
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/IntStream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = &jni.find_class("Javajava/util/OptionalInt")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(I)Ljava/util/OptionalInt;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn empty(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/OptionalInt")?;
        let res = jni.call_static_method(cls, "empty", "()Ljava/util/OptionalInt;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn is_present(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn or_else(&mut self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            "(I)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn or_else_throw(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseThrow",
            "(Ljava/util/function/Supplier;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
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
pub struct JavaTreeMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTreeMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTreeMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaTreeMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaTreeMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaTreeMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/TreeMap")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub unsafe fn new_with_comparator(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/TreeMap")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub fn first_key(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "firstKey", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn navigable_key_set(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "navigableKeySet",
            "()Ljava/util/NavigableSet;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn descending_key_set(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingKeySet",
            "()Ljava/util/NavigableSet;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn descending_map(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingMap",
            "()Ljava/util/NavigableMap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn sub_map_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<bool>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
        arg3: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let val_3 = arg2.unwrap();
        // 1
        let val_4 = jni::objects::JValueGen::Bool(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subMap",
            "(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn head_map_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "headMap",
            "(Ljava/lang/Object;Z)Ljava/util/NavigableMap;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn tail_map_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tailMap",
            "(Ljava/lang/Object;Z)Ljava/util/NavigableMap;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last_key(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "lastKey", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn lower_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lowerKey",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn floor_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floorKey",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn ceiling_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceilingKey",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn higher_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higherKey",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_first_entry(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollFirstEntry",
            "()Ljava/util/Map$Entry;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_last_entry(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollLastEntry",
            "()Ljava/util/Map$Entry;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn first_entry(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "firstEntry",
            "()Ljava/util/Map$Entry;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last_entry(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastEntry",
            "()Ljava/util/Map$Entry;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn lower_entry(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lowerEntry",
            "(Ljava/lang/Object;)Ljava/util/Map$Entry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn floor_entry(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floorEntry",
            "(Ljava/lang/Object;)Ljava/util/Map$Entry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn ceiling_entry(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceilingEntry",
            "(Ljava/lang/Object;)Ljava/util/Map$Entry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn higher_entry(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higherEntry",
            "(Ljava/lang/Object;)Ljava/util/Map$Entry;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn comparator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "comparator",
            "()Ljava/util/Comparator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
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
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaTreeMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaDeque. Needed for returning it from Java.
pub struct JavaDeque<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaDeque<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDeque from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaDeque")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDeque object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn push(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pop", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn poll_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn offer_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_first(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFirst", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "peekFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_first_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_last(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLast",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peekLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_last_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn descending_iterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn poll(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "poll", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peek", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn element(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "element", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaDeque<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaDeque<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaFormattable. Needed for returning it from Java.
pub struct JavaFormattable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaFormattable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaFormattable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaFormattable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFormattable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn format_to(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "formatTo",
            "(Ljava/util/Formatter;III)V",
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
impl<'mc> JNIRaw<'mc> for JavaFormattable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaOptionalDouble<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalDouble<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalDouble from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaOptionalDouble")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaOptionalDouble object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn as_double(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsDouble", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub unsafe fn if_present_or_else(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresentOrElse",
            "(Ljava/util/function/DoubleConsumer;Ljava/lang/Runnable;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn or_else_get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            "(Ljava/util/function/DoubleSupplier;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub unsafe fn if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            "(Ljava/util/function/DoubleConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn equals(
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/DoubleStream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("Javajava/util/OptionalDouble")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(D)Ljava/util/OptionalDouble;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn empty(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/OptionalDouble")?;
        let res = jni.call_static_method(cls, "empty", "()Ljava/util/OptionalDouble;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn is_present(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn or_else(&mut self, arg0: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            "(D)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub unsafe fn or_else_throw(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseThrow",
            "(Ljava/util/function/Supplier;)D",
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
pub struct JavaOptionalLong<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptionalLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptionalLong<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalLong from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaOptionalLong")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaOptionalLong object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn as_long(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsLong", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub unsafe fn if_present_or_else(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresentOrElse",
            "(Ljava/util/function/LongConsumer;Ljava/lang/Runnable;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn or_else_get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            "(Ljava/util/function/LongSupplier;)J",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub unsafe fn if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            "(Ljava/util/function/LongConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn equals(
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/LongStream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let cls = &jni.find_class("Javajava/util/OptionalLong")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(J)Ljava/util/OptionalLong;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn empty(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/OptionalLong")?;
        let res = jni.call_static_method(cls, "empty", "()Ljava/util/OptionalLong;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn is_present(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn or_else(&mut self, arg0: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            "(J)J",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub unsafe fn or_else_throw(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseThrow",
            "(Ljava/util/function/Supplier;)J",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
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
pub struct JavaLinkedHashSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLinkedHashSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLinkedHashSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedHashSet from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaLinkedHashSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLinkedHashSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/LinkedHashSet")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/LinkedHashSet")?;
        let res = jni.new_object(
            cls,
            "(IF)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
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
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaLinkedHashSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaHashMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaHashMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaHashMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/HashMap")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/HashMap")?;
        let res = jni.new_object(
            cls,
            "(IF)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaHashMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaIdentityHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaIdentityHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaIdentityHashMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIdentityHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaIdentityHashMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIdentityHashMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/IdentityHashMap")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
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
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaIdentityHashMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaLinkedHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLinkedHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLinkedHashMap<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaLinkedHashMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLinkedHashMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/LinkedHashMap")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<bool>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        // 1
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let cls = &jni.find_class("Javajava/util/LinkedHashMap")?;
        let res = jni.new_object(
            cls,
            "(IFZ)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn values(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "values",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/BiFunction;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn entry_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "entrySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/BiConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn key_set(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "keySet", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn get_or_default(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn put(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_with_object(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replace",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn merge(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let res = self.jni_ref().call_method(&self.jni_object(),"merge","(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn put_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn put_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_key(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn compute_if_absent(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compute_if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaLinkedHashMap<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractSequentialList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractSequentialList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractSequentialList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractSequentialList from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractSequentialList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractSequentialList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_with_int(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractSequentialList<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaComparator. Needed for returning it from Java.
pub struct JavaComparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaComparator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaComparator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaComparator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaComparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn equals(
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
    pub unsafe fn compare(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compare",
            "(Ljava/lang/Object;Ljava/lang/Object;)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn reverse_order(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(cls, "reverseOrder", "()Ljava/util/Comparator;", &[])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn comparing_with_function(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparing",
            "(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn then_comparing_with_comparator(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparing",
            "(Ljava/util/function/Function;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn then_comparing_with_function(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparing",
            "(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn comparing_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparingInt",
            "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn comparing_long(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparingLong",
            "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn comparing_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparingDouble",
            "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn reversed(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "reversed",
            "()Ljava/util/Comparator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn then_comparing_int(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingInt",
            "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn then_comparing_long(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingLong",
            "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn then_comparing_double(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingDouble",
            "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn natural_order(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(cls, "naturalOrder", "()Ljava/util/Comparator;", &[])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn nulls_first(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "nullsFirst",
            "(Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn nulls_last(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "nullsLast",
            "(Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaComparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaBase64Encoder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64Encoder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64Encoder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBase64Encoder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaBase64Encoder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBase64Encoder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn encode_to_string(
        &mut self,
        arg0: Vec<i8>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "encodeToString",
            "(B)Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn without_padding(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "withoutPadding",
            "()Ljava/util/Base64$Encoder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn wrap(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wrap",
            "(Ljava/io/OutputStream;)Ljava/io/OutputStream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn encode_with_bytes(
        &mut self,
        arg0: Vec<i8>,
        arg1: std::option::Option<Vec<i8>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "encode", "(BB)I", &[]);
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
    pub unsafe fn equals(
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
pub struct JavaRandom<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaRandom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaRandom<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaRandom from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandom")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandom object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Random")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBoolean", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextLong",
            "(JJ)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextFloat",
            "(FF)F",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(II)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ints",
            "(JII)Ljava/util/stream/IntStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "longs",
            "(JJJ)Ljava/util/stream/LongStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(DD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "doubles",
            "(JDD)Ljava/util/stream/DoubleStream;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", "(B)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextGaussian",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextInt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "nextDouble",
            "(DD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
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
    pub unsafe fn equals(
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
    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDeprecated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextExponential", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandom<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaList. Needed for returning it from Java.
pub struct JavaList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn copy_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/List")?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            "(Ljava/util/Collection;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/List")?;
        let res = jni.call_static_method(cls, "of", "(Ljava/lang/Object;)Ljava/util/List;", &[])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of_with_object(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
        arg3: jni::objects::JObject<'mc>,
        arg4: jni::objects::JObject<'mc>,
        arg5: jni::objects::JObject<'mc>,
        arg6: jni::objects::JObject<'mc>,
        arg7: jni::objects::JObject<'mc>,
        arg8: jni::objects::JObject<'mc>,
        arg9: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let val_4 = arg3;
        let val_5 = arg4;
        let val_6 = arg5;
        let val_7 = arg6;
        let val_8 = arg7;
        let val_9 = arg8;
        let val_10 = arg9.unwrap();
        let cls = &jni.find_class("Javajava/util/List")?;
        let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8),jni::objects::JValueGen::from(&val_9),jni::objects::JValueGen::from(&val_10)])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaList<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaUUID<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaUUID<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaUUID<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaUUID from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaUUID")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaUUID object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn least_significant_bits(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeastSignificantBits", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub fn most_significant_bits(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMostSignificantBits", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub fn clock_sequence(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clockSequence", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn variant(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "variant", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn version(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "version", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<u128>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let upper = (arg0.unwrap() >> 64) as u64 as i64;
        let lower = arg0.unwrap() as u64 as i64;
        let val_1 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/util/UUID;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn timestamp(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "timestamp", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub fn node(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "node", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
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
pub struct JavaHashSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaHashSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaHashSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaHashSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaHashSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/HashSet")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Collection;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<f32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let cls = &jni.find_class("Javajava/util/HashSet")?;
        let res = jni.new_object(
            cls,
            "(IF)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res)
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
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
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaHashSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaAbstractList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractList from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove_with_int(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaAbstractList<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaOptional<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaOptional<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaOptional<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaOptional from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaOptional")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaOptional object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn if_present_or_else(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresentOrElse",
            "(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn or(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            "(Ljava/util/function/Supplier;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn or_else_get(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            "(Ljava/util/function/Supplier;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of_nullable(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Optional")?;
        let res = jni.call_static_method(
            cls,
            "ofNullable",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn if_present(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "get", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn map(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "map",
            "(Ljava/util/function/Function;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Optional")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn filter(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "filter",
            "(Ljava/util/function/Predicate;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn empty(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("Javajava/util/Optional")?;
        let res = jni.call_static_method(cls, "empty", "()Ljava/util/Optional;", &[])?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn flat_map(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "flatMap",
            "(Ljava/util/function/Function;)Ljava/util/Optional;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn is_present(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn or_else(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn or_else_throw(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseThrow",
            "(Ljava/util/function/Supplier;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
/// An instantiatable struct that implements JavaListIterator. Needed for returning it from Java.
pub struct JavaListIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaListIterator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaListIterator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaListIterator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaListIterator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next_index(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextIndex", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn previous_index(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "previousIndex", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn has_previous(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasPrevious", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn previous(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "previous", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each_remaining(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaListIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaListIterator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaLinkedList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLinkedList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLinkedList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedList from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaLinkedList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLinkedList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/LinkedList")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Collection;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res)
    }
    pub unsafe fn push(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pop", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn poll_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn offer_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_first(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFirst", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "peekFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_first_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_last(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLast",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peekLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_last_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn descending_iterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_with_object(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(ILjava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn get(
        &mut self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(I)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn last_index_of(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all_with_collection(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn set(
        &mut self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "poll", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peek", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn element(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "element", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn list_iterator(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn sub_list(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            "(II)Ljava/util/List;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn replace_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            "(Ljava/util/function/UnaryOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn sort(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            "(Ljava/util/Comparator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaLinkedList<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaCollection. Needed for returning it from Java.
pub struct JavaCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaCollection<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaCollection from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaCollection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCollection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn equals(
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for JavaCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaPrimitiveIteratorOfInt. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfInt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaPrimitiveIteratorOfInt<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfInt from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfInt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPrimitiveIteratorOfInt object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn next_int(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn for_each_remaining_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each_remaining_with_int_consumer(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/IntConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaPrimitiveIteratorOfLong. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfLong<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaPrimitiveIteratorOfLong<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfLong from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfLong")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPrimitiveIteratorOfLong object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn next_long(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub unsafe fn for_each_remaining_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each_remaining_with_long_consumer(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/LongConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaIterator. Needed for returning it from Java.
pub struct JavaIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaIterator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaIterator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaIterator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIterator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each_remaining(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaPrimitiveIterator. Needed for returning it from Java.
pub struct JavaPrimitiveIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaPrimitiveIterator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIterator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIterator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPrimitiveIterator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn for_each_remaining_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaPrimitiveIterator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
/// An instantiatable struct that implements JavaMapEntry. Needed for returning it from Java.
pub struct JavaMapEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaMapEntry<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaMapEntry from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaMapEntry")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaMapEntry object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn comparing_by_key(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparingByKey",
            "(Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn comparing_by_value(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/Comparator")?;
        let res = jni.call_static_method(
            cls,
            "comparingByValue",
            "(Ljava/util/Comparator;)Ljava/util/Comparator;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn copy_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Map$Entry")?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            "(Ljava/util/Map$Entry;)Ljava/util/Map$Entry;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn value(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getValue", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn key(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKey", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn set_value(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaMapEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaEnumSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaEnumSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaEnumSet<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaEnumSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaEnumSet")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaEnumSet object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn all_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(
            cls,
            "allOf",
            "(Ljava/lang/Class;)Ljava/util/EnumSet;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn complement_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(
            cls,
            "complementOf",
            "(Ljava/util/EnumSet;)Ljava/util/EnumSet;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn range(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(
            cls,
            "range",
            "(Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/util/EnumSet;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn copy_of_with_collection(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            "(Ljava/util/EnumSet;)Ljava/util/EnumSet;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn of_with_enum(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
        arg3: jni::objects::JObject<'mc>,
        arg4: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let val_4 = arg3;
        let val_5 = arg4.unwrap();
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(cls,"of",
"(Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        Ok(res.l().unwrap())
    }
    pub fn none_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/EnumSet")?;
        let res = jni.call_static_method(
            cls,
            "noneOf",
            "(Ljava/lang/Class;)Ljava/util/EnumSet;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn equals(
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
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
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
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaEnumSet<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaArrayDeque<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaArrayDeque<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaArrayDeque<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaArrayDeque from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaArrayDeque")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaArrayDeque object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/ArrayDeque")?;
        let res = jni.new_object(cls, "(I)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    pub unsafe fn push(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pop", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn add_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn poll_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pollFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn poll_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn offer_last(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_first(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFirst", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_first(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "peekFirst",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_first_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer_first(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_last(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLast",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek_last(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peekLast", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_last_occurrence(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn offer(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn descending_iterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn remove(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Ljava/util/ArrayDeque;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn iterator(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "iterator",
            "()Ljava/util/Iterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn contains(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn spliterator(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spliterator",
            "()Ljava/util/Spliterator;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn add_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn for_each(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn poll(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "poll", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn peek(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "peek", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_if(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            "(Ljava/util/function/Predicate;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn element(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "element", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn remove_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn retain_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
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
    pub unsafe fn contains_all(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
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
    pub unsafe fn equals(
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
    pub fn stream(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "stream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn parallel_stream(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parallelStream",
            "()Ljava/util/stream/Stream;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaArrayDeque<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
pub struct JavaDate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaDate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaDate<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaDate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Date")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_1)])?;
        Ok(res)
    }
    #[deprecated]
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
        let cls = &jni.find_class("Javajava/util/Date")?;
        let res = jni.new_object(
            cls,
            "(IIIIII)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        Ok(res)
    }
    #[deprecated]
    pub fn set_date(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDate",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn month(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMonth", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn set_month(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMonth",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn hours(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHours", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn set_hours(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHours",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn minutes(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinutes", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn set_minutes(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinutes",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn set_seconds(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeconds",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn set_year(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYear",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    pub fn date(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDate", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn day(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDay", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn to_locale_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toLocaleString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    pub fn to_gmtstring(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toGMTString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    pub fn timezone_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTimezoneOffset", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn time(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTime", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    #[deprecated]
    pub fn year(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYear", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn seconds(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeconds", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn to_instant(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toInstant",
            "()Ljava/time/Instant;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    #[deprecated]
    pub fn utc(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.into());
        let cls = &jni.find_class("Javalong")?;
        let res = jni.call_static_method(
            cls,
            "UTC",
            "(IIIIII)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        Ok(res.j().unwrap())
    }
    pub unsafe fn before(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "before",
            "(Ljava/util/Date;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub unsafe fn after(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "after",
            "(Ljava/util/Date;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn parse(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("Javalong")?;
        let res = jni.call_static_method(
            cls,
            "parse",
            "(Ljava/lang/String;)J",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.j().unwrap())
    }
    pub fn set_time(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTime",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn equals(
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
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub unsafe fn compare_to_with_date(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn from(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("Javajava/util/Date")?;
        let res = jni.call_static_method(
            cls,
            "from",
            "(Ljava/time/Instant;)Ljava/util/Date;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.l().unwrap())
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
/// An instantiatable struct that implements JavaPrimitiveIteratorOfDouble. Needed for returning it from Java.
pub struct JavaPrimitiveIteratorOfDouble<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaPrimitiveIteratorOfDouble<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfDouble from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaPrimitiveIteratorOfDouble")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPrimitiveIteratorOfDouble object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn next_double(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub unsafe fn for_each_remaining_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/Consumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub unsafe fn for_each_remaining_with_double_consumer(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            "(Ljava/util/function/DoubleConsumer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn next(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "next", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaBase64Decoder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaBase64Decoder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaBase64Decoder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBase64Decoder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaBase64Decoder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBase64Decoder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn decode_with_bytes(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "decode",
            "(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    pub fn decode_with_string(
        &mut self,
        arg0: std::option::Option<Vec<i8>>,
        arg1: std::option::Option<Vec<i8>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "decode", "(BB)I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub unsafe fn wrap(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wrap",
            "(Ljava/io/InputStream;)Ljava/io/InputStream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
    pub unsafe fn equals(
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
pub mod logging;
pub mod random;
pub mod regex;
