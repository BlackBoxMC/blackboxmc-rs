#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Resizable-array implementation of the <tt>List</tt> interface. Implements all optional list operations, and permits all elements, including <tt>null</tt>. In addition to implementing the <tt>List</tt> interface, this class provides methods to manipulate the size of the array that is used internally to store the list. (This class is roughly equivalent to <tt>Vector</tt>, except that it is unsynchronized.)
/// <p>The <tt>size</tt>, <tt>isEmpty</tt>, <tt>get</tt>, <tt>set</tt>, <tt>iterator</tt>, and <tt>listIterator</tt> operations run in constant time. The <tt>add</tt> operation runs in <i>amortized constant time</i>, that is, adding n elements requires O(n) time. All of the other operations run in linear time (roughly speaking). The constant factor is low compared to that for the <tt>LinkedList</tt> implementation.</p>
/// <p>Each <tt>ArrayList</tt> instance has a <i>capacity</i>. The capacity is the size of the array used to store the elements in the list. It is always at least as large as the list size. As elements are added to an ArrayList, its capacity grows automatically. The details of the growth policy are not specified beyond the fact that adding an element has constant amortized time cost.</p>
/// <p>An application can increase the capacity of an <tt>ArrayList</tt> instance before adding a large number of elements using the <tt>ensureCapacity</tt> operation. This may reduce the amount of incremental reallocation.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access an <tt>ArrayList</tt> instance concurrently, and at least one of the threads modifies the list structurally, it <i>must</i> be synchronized externally. (A structural modification is any operation that adds or deletes one or more elements, or explicitly resizes the backing array; merely setting the value of an element is not a structural modification.) This is typically accomplished by synchronizing on some object that naturally encapsulates the list. If no such object exists, the list should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedList-java.util.List-"><code>Collections.synchronizedList</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the list:</p>
/// <pre> List list = Collections.synchronizedList(new ArrayList(...));</pre>
/// <p><a name="fail-fast"> The iterators returned by this class's </a><a href="../../java/util/ArrayList.html#iterator--"><code>iterator</code></a> and <a href="../../java/util/ArrayList.html#listIterator-int-"><code>listIterator</code></a> methods are <em>fail-fast</em>: if the list is structurally modified at any time after the iterator is created, in any way except through the iterator's own <a href="../../java/util/ListIterator.html#remove--"><code>remove</code></a> or <a href="../../java/util/ListIterator.html#add-E-"><code>add</code></a> methods, the iterator will throw a <a title="class in java.util" href="../../java/util/ConcurrentModificationException.html"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaArrayList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaArrayList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaArrayList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaArrayList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/ArrayList")?;
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
}

impl<'mc> JavaArrayList<'mc> {
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_int(
        &self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn ensure_capacity(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ensureCapacity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn trim_to_size(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "trimToSize", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaArrayList<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaArrayList.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaList<'mc>> for JavaArrayList<'mc> {
    fn into(self) -> crate::util::JavaList<'mc> {
        crate::util::JavaList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaArrayList into crate::util::JavaList")
    }
}
impl<'mc> Into<crate::util::JavaRandomAccess<'mc>> for JavaArrayList<'mc> {
    fn into(self) -> crate::util::JavaRandomAccess<'mc> {
        crate::util::JavaRandomAccess::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaArrayList into crate::util::JavaRandomAccess")
    }
}
impl<'mc> Into<crate::util::JavaAbstractList<'mc>> for JavaArrayList<'mc> {
    fn into(self) -> crate::util::JavaAbstractList<'mc> {
        crate::util::JavaAbstractList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaArrayList into crate::util::JavaAbstractList")
    }
}
/// This class provides a skeletal implementation of the <tt>Map</tt> interface, to minimize the effort required to implement this interface.
/// <p>To implement an unmodifiable map, the programmer needs only to extend this class and provide an implementation for the <tt>entrySet</tt> method, which returns a set-view of the map's mappings. Typically, the returned set will, in turn, be implemented atop <tt>AbstractSet</tt>. This set should not support the <tt>add</tt> or <tt>remove</tt> methods, and its iterator should not support the <tt>remove</tt> method.</p>
/// <p>To implement a modifiable map, the programmer must additionally override this class's <tt>put</tt> method (which otherwise throws an <tt>UnsupportedOperationException</tt>), and the iterator returned by <tt>entrySet().iterator()</tt> must additionally implement its <tt>remove</tt> method.</p>
/// <p>The programmer should generally provide a void (no argument) and map constructor, as per the recommendation in the <tt>Map</tt> interface specification.</p>
/// <p>The documentation for each non-abstract method in this class describes its implementation in detail. Each of these methods may be overridden if the map being implemented admits a more efficient implementation.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractMap")?;
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
}

impl<'mc> JavaAbstractMap<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}

impl<'mc> std::string::ToString for JavaAbstractMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaAbstractMap<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractMap into crate::util::JavaMap")
    }
}
/// A <a title="interface in java.util" href="../../java/util/NavigableSet.html"><code>NavigableSet</code></a> implementation based on a <a href="../../java/util/TreeMap.html" title="class in java.util"><code>TreeMap</code></a>. The elements are ordered using their <a title="interface in java.lang" href="../../java/lang/Comparable.html">natural ordering</a>, or by a <a href="../../java/util/Comparator.html" title="interface in java.util"><code>Comparator</code></a> provided at set creation time, depending on which constructor is used.
/// <p>This implementation provides guaranteed log(n) time cost for the basic operations (<code>add</code>, <code>remove</code> and <code>contains</code>).</p>
/// <p>Note that the ordering maintained by a set (whether or not an explicit comparator is provided) must be <i>consistent with equals</i> if it is to correctly implement the <code>Set</code> interface. (See <code>Comparable</code> or <code>Comparator</code> for a precise definition of <i>consistent with equals</i>.) This is so because the <code>Set</code> interface is defined in terms of the <code>equals</code> operation, but a <code>TreeSet</code> instance performs all element comparisons using its <code>compareTo</code> (or <code>compare</code>) method, so two elements that are deemed equal by this method are, from the standpoint of the set, equal. The behavior of a set <i>is</i> well-defined even if its ordering is inconsistent with equals; it just fails to obey the general contract of the <code>Set</code> interface.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a tree set concurrently, and at least one of the threads modifies the set, it <i>must</i> be synchronized externally. This is typically accomplished by synchronizing on some object that naturally encapsulates the set. If no such object exists, the set should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedSortedSet-java.util.SortedSet-"><code>Collections.synchronizedSortedSet</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the set:</p>
/// <pre> SortedSet s = Collections.synchronizedSortedSet(new TreeSet(...));</pre>
/// <p>The iterators returned by this class's <code>iterator</code> method are <i>fail-fast</i>: if the set is modified at any time after the iterator is created, in any way except through the iterator's own <code>remove</code> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaTreeSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaTreeSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaTreeSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaTreeSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/TreeSet")?;
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
}

impl<'mc> JavaTreeSet<'mc> {
    //

    pub fn poll_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn descending_iterator(
        &self,
    ) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ceiling(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceiling",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn higher(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higher",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn floor(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "last", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "first", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn lower(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn comparator(
        &self,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Comparator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "comparator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaTreeSet<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaTreeSet.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaAbstractSet<'mc>> for JavaTreeSet<'mc> {
    fn into(self) -> crate::util::JavaAbstractSet<'mc> {
        crate::util::JavaAbstractSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaTreeSet into crate::util::JavaAbstractSet")
    }
}
/// The <code>Vector</code> class implements a growable array of objects. Like an array, it contains components that can be accessed using an integer index. However, the size of a <code>Vector</code> can grow or shrink as needed to accommodate adding and removing items after the <code>Vector</code> has been created.
/// <p>Each vector tries to optimize storage management by maintaining a <code>capacity</code> and a <code>capacityIncrement</code>. The <code>capacity</code> is always at least as large as the vector size; it is usually larger because as components are added to the vector, the vector's storage increases in chunks the size of <code>capacityIncrement</code>. An application can increase the capacity of a vector before inserting a large number of components; this reduces the amount of incremental reallocation.</p>
/// <p><a name="fail-fast"> The iterators returned by this class's </a><a href="../../java/util/Vector.html#iterator--"><code>iterator</code></a> and <a href="../../java/util/Vector.html#listIterator-int-"><code>listIterator</code></a> methods are <em>fail-fast</em>: if the vector is structurally modified at any time after the iterator is created, in any way except through the iterator's own <a href="../../java/util/ListIterator.html#remove--"><code>remove</code></a> or <a href="../../java/util/ListIterator.html#add-E-"><code>add</code></a> methods, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future. The <a href="../../java/util/Enumeration.html" title="interface in java.util"><code>Enumerations</code></a> returned by the <a href="../../java/util/Vector.html#elements--"><code>elements</code></a> method are <em>not</em> fail-fast.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>As of the Java 2 platform v1.2, this class was retrofitted to implement the <a href="../../java/util/List.html" title="interface in java.util"><code>List</code></a> interface, making it a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>. Unlike the new collection implementations, <code>Vector</code> is synchronized. If a thread-safe implementation is not needed, it is recommended to use <a href="../../java/util/ArrayList.html" title="class in java.util"><code>ArrayList</code></a> in place of <code>Vector</code>.</p>
pub struct JavaVector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaVector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaVector<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaVector from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Vector")?;
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
}

impl<'mc> JavaVector<'mc> {
    //

    pub fn remove_element_at(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeElementAt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_element(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeElement",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn insert_element_at(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;I)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "insertElementAt",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_all_elements(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAllElements",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn first_element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "firstElement", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn last_element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "lastElement", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn set_element_at(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;I)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setElementAt",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_element(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addElement",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_size(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn copy_into(
        &self,
        arg0: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copyInto", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn index_of_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "indexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lastIndexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn elements(
        &self,
    ) -> Result<crate::util::JavaEnumeration<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Enumeration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "elements", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaEnumeration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn capacity(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "capacity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn ensure_capacity(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ensureCapacity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn trim_to_size(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "trimToSize", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn element_at(
        &self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "elementAt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaVector<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaVector.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaList<'mc>> for JavaVector<'mc> {
    fn into(self) -> crate::util::JavaList<'mc> {
        crate::util::JavaList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaVector into crate::util::JavaList")
    }
}
impl<'mc> Into<crate::util::JavaRandomAccess<'mc>> for JavaVector<'mc> {
    fn into(self) -> crate::util::JavaRandomAccess<'mc> {
        crate::util::JavaRandomAccess::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaVector into crate::util::JavaRandomAccess")
    }
}
impl<'mc> Into<crate::util::JavaAbstractList<'mc>> for JavaVector<'mc> {
    fn into(self) -> crate::util::JavaAbstractList<'mc> {
        crate::util::JavaAbstractList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaVector into crate::util::JavaAbstractList")
    }
}
/// Marker interface used by <tt>List</tt> implementations to indicate that they support fast (generally constant time) random access. The primary purpose of this interface is to allow generic algorithms to alter their behavior to provide good performance when applied to either random or sequential access lists.
/// <p>The best algorithms for manipulating random access lists (such as <tt>ArrayList</tt>) can produce quadratic behavior when applied to sequential access lists (such as <tt>LinkedList</tt>). Generic list algorithms are encouraged to check whether the given list is an <tt>instanceof</tt> this interface before applying an algorithm that would provide poor performance if it were applied to a sequential access list, and to alter their behavior if necessary to guarantee acceptable performance.</p>
/// <p>It is recognized that the distinction between random and sequential access is often fuzzy. For example, some <tt>List</tt> implementations provide asymptotically linear access times if they get huge, but constant access times in practice. Such a <tt>List</tt> implementation should generally implement this interface. As a rule of thumb, a <tt>List</tt> implementation should implement this interface if, for typical instances of the class, this loop:</p>
/// <pre> for (int i=0, n=list.size(); i &lt; n; i++)
/// list.get(i);
/// </pre> runs faster than this loop:
/// <pre> for (Iterator i=list.iterator(); i.hasNext(); )
/// i.next();
/// </pre>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaRandomAccess<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandomAccess<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandomAccess<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaRandomAccess from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/RandomAccess")?;
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

impl<'mc> JavaRandomAccess<'mc> {}
/// An object that maps keys to values. A map cannot contain duplicate keys; each key can map to at most one value.
/// <p>This interface takes the place of the <tt>Dictionary</tt> class, which was a totally abstract class rather than an interface.</p>
/// <p>The <tt>Map</tt> interface provides three <i>collection views</i>, which allow a map's contents to be viewed as a set of keys, collection of values, or set of key-value mappings. The <i>order</i> of a map is defined as the order in which the iterators on the map's collection views return their elements. Some map implementations, like the <tt>TreeMap</tt> class, make specific guarantees as to their order; others, like the <tt>HashMap</tt> class, do not.</p>
/// <p>Note: great care must be exercised if mutable objects are used as map keys. The behavior of a map is not specified if the value of an object is changed in a manner that affects <tt>equals</tt> comparisons while the object is a key in the map. A special case of this prohibition is that it is not permissible for a map to contain itself as a key. While it is permissible for a map to contain itself as a value, extreme caution is advised: the <tt>equals</tt> and <tt>hashCode</tt> methods are no longer well defined on such a map.</p>
/// <p>All general-purpose map implementation classes should provide two "standard" constructors: a void (no arguments) constructor which creates an empty map, and a constructor with a single argument of type <tt>Map</tt>, which creates a new map with the same key-value mappings as its argument. In effect, the latter constructor allows the user to copy any map, producing an equivalent map of the desired class. There is no way to enforce this recommendation (as interfaces cannot contain constructors) but all of the general-purpose map implementations in the JDK comply.</p>
/// <p>The "destructive" methods contained in this interface, that is, the methods that modify the map on which they operate, are specified to throw <tt>UnsupportedOperationException</tt> if this map does not support the operation. If this is the case, these methods may, but are not required to, throw an <tt>UnsupportedOperationException</tt> if the invocation would have no effect on the map. For example, invoking the <a href="../../java/util/Map.html#putAll-java.util.Map-"><code>putAll(Map)</code></a> method on an unmodifiable map may, but is not required to, throw the exception if the map whose mappings are to be "superimposed" is empty.</p>
/// <p>Some map implementations have restrictions on the keys and values they may contain. For example, some implementations prohibit null keys and values, and some have restrictions on the types of their keys. Attempting to insert an ineligible key or value throws an unchecked exception, typically <tt>NullPointerException</tt> or <tt>ClassCastException</tt>. Attempting to query the presence of an ineligible key or value may throw an exception, or it may simply return false; some implementations will exhibit the former behavior and some will exhibit the latter. More generally, attempting an operation on an ineligible key or value whose completion would not result in the insertion of an ineligible element into the map may throw an exception or it may succeed, at the option of the implementation. Such exceptions are marked as "optional" in the specification for this interface.</p>
/// <p>Many methods in Collections Framework interfaces are defined in terms of the <a href="../../java/lang/Object.html#equals-java.lang.Object-"><code>equals</code></a> method. For example, the specification for the <a href="../../java/util/Map.html#containsKey-java.lang.Object-"><code>containsKey(Object key)</code></a> method says: "returns <tt>true</tt> if and only if this map contains a mapping for a key <tt>k</tt> such that <tt>(key==null ? k==null : key.equals(k))</tt>." This specification should <i>not</i> be construed to imply that invoking <tt>Map.containsKey</tt> with a non-null argument <tt>key</tt> will cause <tt>key.equals(k)</tt> to be invoked for any key <tt>k</tt>. Implementations are free to implement optimizations whereby the <tt>equals</tt> invocation is avoided, for example, by first comparing the hash codes of the two keys. (The <a href="../../java/lang/Object.html#hashCode--"><code>Object.hashCode()</code></a> specification guarantees that two objects with unequal hash codes cannot be equal.) More generally, implementations of the various Collections Framework interfaces are free to take advantage of the specified behavior of underlying <a href="../../java/lang/Object.html" title="class in java.lang"><code>Object</code></a> methods wherever the implementor deems it appropriate.</p>
/// <p>Some map operations which perform recursive traversal of the map may fail with an exception for self-referential instances where the map directly or indirectly contains itself. This includes the <code>clone()</code>, <code>equals()</code>, <code>hashCode()</code> and <code>toString()</code> methods. Implementations may optionally handle the self-referential scenario, however most current implementations do not do so.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Map")?;
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
}

impl<'mc> JavaMap<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn copy_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<crate::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Map");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaMap::from_raw(&jni, obj)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn of_with_object(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
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
    ) -> Result<crate::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Ljava/lang/Object;";
            let val_4 = jni::objects::JValueGen::Object(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "Ljava/lang/Object;";
            let val_5 = jni::objects::JValueGen::Object(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Ljava/lang/Object;";
            let val_6 = jni::objects::JValueGen::Object(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "Ljava/lang/Object;";
            let val_7 = jni::objects::JValueGen::Object(a);
            args.push(val_7);
        }
        if let Some(a) = arg7 {
            sig += "Ljava/lang/Object;";
            let val_8 = jni::objects::JValueGen::Object(a);
            args.push(val_8);
        }
        if let Some(a) = arg8 {
            sig += "Ljava/lang/Object;";
            let val_9 = jni::objects::JValueGen::Object(a);
            args.push(val_9);
        }
        if let Some(a) = arg9 {
            sig += "Ljava/lang/Object;";
            let val_10 = jni::objects::JValueGen::Object(a);
            args.push(val_10);
        }
        if let Some(a) = arg10 {
            sig += "Ljava/lang/Object;";
            let val_11 = jni::objects::JValueGen::Object(a);
            args.push(val_11);
        }
        if let Some(a) = arg11 {
            sig += "Ljava/lang/Object;";
            let val_12 = jni::objects::JValueGen::Object(a);
            args.push(val_12);
        }
        if let Some(a) = arg12 {
            sig += "Ljava/lang/Object;";
            let val_13 = jni::objects::JValueGen::Object(a);
            args.push(val_13);
        }
        if let Some(a) = arg13 {
            sig += "Ljava/lang/Object;";
            let val_14 = jni::objects::JValueGen::Object(a);
            args.push(val_14);
        }
        if let Some(a) = arg14 {
            sig += "Ljava/lang/Object;";
            let val_15 = jni::objects::JValueGen::Object(a);
            args.push(val_15);
        }
        if let Some(a) = arg15 {
            sig += "Ljava/lang/Object;";
            let val_16 = jni::objects::JValueGen::Object(a);
            args.push(val_16);
        }
        if let Some(a) = arg16 {
            sig += "Ljava/lang/Object;";
            let val_17 = jni::objects::JValueGen::Object(a);
            args.push(val_17);
        }
        if let Some(a) = arg17 {
            sig += "Ljava/lang/Object;";
            let val_18 = jni::objects::JValueGen::Object(a);
            args.push(val_18);
        }
        if let Some(a) = arg18 {
            sig += "Ljava/lang/Object;";
            let val_19 = jni::objects::JValueGen::Object(a);
            args.push(val_19);
        }
        if let Some(a) = arg19 {
            sig += "Ljava/lang/Object;";
            let val_20 = jni::objects::JValueGen::Object(a);
            args.push(val_20);
        }
        sig += ")Ljava/util/Map;";
        let cls = jni.find_class("java/util/Map");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "of", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaMap::from_raw(&jni, obj)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn entry(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let cls = jni.find_class("java/util/Map$Entry");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "entry",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaMapEntry::from_raw(&jni, obj)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn of_entries(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<impl Into<crate::util::JavaMapEntry<'mc>>>,
    ) -> Result<crate::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map$Entry;)Ljava/util/Map;");
        let cls = jni.find_class("java/util/Map");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "ofEntries", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaMap::from_raw(&jni, obj)
    }
}
/// Hash table based implementation of the <tt>Map</tt> interface, with <em>weak keys</em>. An entry in a <tt>WeakHashMap</tt> will automatically be removed when its key is no longer in ordinary use. More precisely, the presence of a mapping for a given key will not prevent the key from being discarded by the garbage collector, that is, made finalizable, finalized, and then reclaimed. When a key has been discarded its entry is effectively removed from the map, so this class behaves somewhat differently from other <tt>Map</tt> implementations.
/// <p>Both null values and the null key are supported. This class has performance characteristics similar to those of the <tt>HashMap</tt> class, and has the same efficiency parameters of <em>initial capacity</em> and <em>load factor</em>.</p>
/// <p>Like most collection classes, this class is not synchronized. A synchronized <tt>WeakHashMap</tt> may be constructed using the <a href="../../java/util/Collections.html#synchronizedMap-java.util.Map-"><code>Collections.synchronizedMap</code></a> method.</p>
/// <p>This class is intended primarily for use with key objects whose <tt>equals</tt> methods test for object identity using the <tt>==</tt> operator. Once such a key is discarded it can never be recreated, so it is impossible to do a lookup of that key in a <tt>WeakHashMap</tt> at some later time and be surprised that its entry has been removed. This class will work perfectly well with key objects whose <tt>equals</tt> methods are not based upon object identity, such as <tt>String</tt> instances. With such recreatable key objects, however, the automatic removal of <tt>WeakHashMap</tt> entries whose keys have been discarded may prove to be confusing.</p>
/// <p>The behavior of the <tt>WeakHashMap</tt> class depends in part upon the actions of the garbage collector, so several familiar (though not required) <tt>Map</tt> invariants do not hold for this class. Because the garbage collector may discard keys at any time, a <tt>WeakHashMap</tt> may behave as though an unknown thread is silently removing entries. In particular, even if you synchronize on a <tt>WeakHashMap</tt> instance and invoke none of its mutator methods, it is possible for the <tt>size</tt> method to return smaller values over time, for the <tt>isEmpty</tt> method to return <tt>false</tt> and then <tt>true</tt>, for the <tt>containsKey</tt> method to return <tt>true</tt> and later <tt>false</tt> for a given key, for the <tt>get</tt> method to return a value for a given key but later return <tt>null</tt>, for the <tt>put</tt> method to return <tt>null</tt> and the <tt>remove</tt> method to return <tt>false</tt> for a key that previously appeared to be in the map, and for successive examinations of the key set, the value collection, and the entry set to yield successively smaller numbers of elements.</p>
/// <p>Each key object in a <tt>WeakHashMap</tt> is stored indirectly as the referent of a weak reference. Therefore a key will automatically be removed only after the weak references to it, both inside and outside of the map, have been cleared by the garbage collector.</p>
/// <p><strong>Implementation note:</strong> The value objects in a <tt>WeakHashMap</tt> are held by ordinary strong references. Thus care should be taken to ensure that value objects do not strongly refer to their own keys, either directly or indirectly, since that will prevent the keys from being discarded. Note that a value object may refer indirectly to its key via the <tt>WeakHashMap</tt> itself; that is, a value object may strongly refer to some other key object whose associated value object, in turn, strongly refers to the key of the first value object. If the values in the map do not rely on the map holding strong references to them, one way to deal with this is to wrap values themselves within <tt>WeakReferences</tt> before inserting, as in: <tt>m.put(key, new WeakReference(value))</tt>, and then unwrapping upon each <tt>get</tt>.</p>
/// <p>The iterators returned by the <tt>iterator</tt> method of the collections returned by all of this class's "collection view methods" are <i>fail-fast</i>: if the map is structurally modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaWeakHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaWeakHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaWeakHashMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaWeakHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/WeakHashMap")?;
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
}

impl<'mc> JavaWeakHashMap<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}

impl<'mc> std::string::ToString for JavaWeakHashMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaWeakHashMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaWeakHashMap<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaWeakHashMap into crate::util::JavaMap")
    }
}
impl<'mc> Into<crate::util::JavaAbstractMap<'mc>> for JavaWeakHashMap<'mc> {
    fn into(self) -> crate::util::JavaAbstractMap<'mc> {
        crate::util::JavaAbstractMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaWeakHashMap into crate::util::JavaAbstractMap")
    }
}
/// A collection that contains no duplicate elements. More formally, sets contain no pair of elements <code>e1</code> and <code>e2</code> such that <code>e1.equals(e2)</code>, and at most one null element. As implied by its name, this interface models the mathematical <i>set</i> abstraction.
/// <p>The <tt>Set</tt> interface places additional stipulations, beyond those inherited from the <tt>Collection</tt> interface, on the contracts of all constructors and on the contracts of the <tt>add</tt>, <tt>equals</tt> and <tt>hashCode</tt> methods. Declarations for other inherited methods are also included here for convenience. (The specifications accompanying these declarations have been tailored to the <tt>Set</tt> interface, but they do not contain any additional stipulations.)</p>
/// <p>The additional stipulation on constructors is, not surprisingly, that all constructors must create a set that contains no duplicate elements (as defined above).</p>
/// <p>Note: Great care must be exercised if mutable objects are used as set elements. The behavior of a set is not specified if the value of an object is changed in a manner that affects <tt>equals</tt> comparisons while the object is an element in the set. A special case of this prohibition is that it is not permissible for a set to contain itself as an element.</p>
/// <p>Some set implementations have restrictions on the elements that they may contain. For example, some implementations prohibit null elements, and some have restrictions on the types of their elements. Attempting to add an ineligible element throws an unchecked exception, typically <tt>NullPointerException</tt> or <tt>ClassCastException</tt>. Attempting to query the presence of an ineligible element may throw an exception, or it may simply return false; some implementations will exhibit the former behavior and some will exhibit the latter. More generally, attempting an operation on an ineligible element whose completion would not result in the insertion of an ineligible element into the set may throw an exception or it may succeed, at the option of the implementation. Such exceptions are marked as "optional" in the specification for this interface.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Set")?;
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
}

impl<'mc> JavaSet<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn copy_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Ljava/util/Set;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Set");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaSet::from_raw(&jni, obj)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn of_with_object(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
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
    ) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Ljava/lang/Object;";
            let val_4 = jni::objects::JValueGen::Object(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "Ljava/lang/Object;";
            let val_5 = jni::objects::JValueGen::Object(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Ljava/lang/Object;";
            let val_6 = jni::objects::JValueGen::Object(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "Ljava/lang/Object;";
            let val_7 = jni::objects::JValueGen::Object(a);
            args.push(val_7);
        }
        if let Some(a) = arg7 {
            sig += "Ljava/lang/Object;";
            let val_8 = jni::objects::JValueGen::Object(a);
            args.push(val_8);
        }
        if let Some(a) = arg8 {
            sig += "Ljava/lang/Object;";
            let val_9 = jni::objects::JValueGen::Object(a);
            args.push(val_9);
        }
        if let Some(a) = arg9 {
            sig += "Ljava/lang/Object;";
            let val_10 = jni::objects::JValueGen::Object(a);
            args.push(val_10);
        }
        sig += ")Ljava/util/Set;";
        let cls = jni.find_class("java/util/Set");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "of", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaSet::from_raw(&jni, obj)
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::util::JavaCollection<'mc>> for JavaSet<'mc> {
    fn into(self) -> crate::util::JavaCollection<'mc> {
        crate::util::JavaCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaSet into crate::util::JavaCollection")
    }
}
/// This class consists exclusively of static methods for obtaining encoders and decoders for the Base64 encoding scheme. The implementation of this class supports the following types of Base64 as specified in <a href="http://www.ietf.org/rfc/rfc4648.txt">RFC 4648</a> and <a href="http://www.ietf.org/rfc/rfc2045.txt">RFC 2045</a>.
/// <ul>
/// <li><a name="basic"><b>Basic</b></a>
/// <p>Uses "The Base64 Alphabet" as specified in Table 1 of RFC 4648 and RFC 2045 for encoding and decoding operation. The encoder does not add any line feed (line separator) character. The decoder rejects data that contains characters outside the base64 alphabet.</p></li>
/// <li><a name="url"><b>URL and Filename safe</b></a>
/// <p>Uses the "URL and Filename safe Base64 Alphabet" as specified in Table 2 of RFC 4648 for encoding and decoding. The encoder does not add any line feed (line separator) character. The decoder rejects data that contains characters outside the base64 alphabet.</p></li>
/// <li><a name="mime"><b>MIME</b></a>
/// <p>Uses the "The Base64 Alphabet" as specified in Table 1 of RFC 2045 for encoding and decoding operation. The encoded output must be represented in lines of no more than 76 characters each and uses a carriage return <code>'\r'</code> followed immediately by a linefeed <code>'\n'</code> as the line separator. No line separator is added to the end of the encoded output. All line separators or other characters not found in the base64 alphabet table are ignored in decoding operation.</p></li>
/// </ul>
/// <p>Unless otherwise noted, passing a <code>null</code> argument to a method of this class will cause a <a href="../../java/lang/NullPointerException.html" title="class in java.lang"><code>NullPointerException</code></a> to be thrown.</p>
pub struct JavaBase64<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBase64<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBase64<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaBase64 from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Base64")?;
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
}

impl<'mc> JavaBase64<'mc> {
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaBase64<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaBase64.toString: {}", err),
        }
    }
}

/// This class provides a skeletal implementation of the <tt>Collection</tt> interface, to minimize the effort required to implement this interface.
/// <p>To implement an unmodifiable collection, the programmer needs only to extend this class and provide implementations for the <tt>iterator</tt> and <tt>size</tt> methods. (The iterator returned by the <tt>iterator</tt> method must implement <tt>hasNext</tt> and <tt>next</tt>.)</p>
/// <p>To implement a modifiable collection, the programmer must additionally override this class's <tt>add</tt> method (which otherwise throws an <tt>UnsupportedOperationException</tt>), and the iterator returned by the <tt>iterator</tt> method must additionally implement its <tt>remove</tt> method.</p>
/// <p>The programmer should generally provide a void (no argument) and <tt>Collection</tt> constructor, as per the recommendation in the <tt>Collection</tt> interface specification.</p>
/// <p>The documentation for each non-abstract method in this class describes its implementation in detail. Each of these methods may be overridden if the collection being implemented admits a more efficient implementation.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractCollection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractCollection from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractCollection")?;
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
}

impl<'mc> JavaAbstractCollection<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaAbstractCollection<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractCollection.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaCollection<'mc>> for JavaAbstractCollection<'mc> {
    fn into(self) -> crate::util::JavaCollection<'mc> {
        crate::util::JavaCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractCollection into crate::util::JavaCollection")
    }
}
/// This class provides skeletal implementations of some <a href="../../java/util/Queue.html" title="interface in java.util"><code>Queue</code></a> operations. The implementations in this class are appropriate when the base implementation does <em>not</em> allow <tt>null</tt> elements. Methods <a href="../../java/util/AbstractQueue.html#add-E-"><code>add</code></a>, <a href="../../java/util/AbstractQueue.html#remove--"><code>remove</code></a>, and <a href="../../java/util/AbstractQueue.html#element--"><code>element</code></a> are based on <a href="../../java/util/Queue.html#offer-E-"><code>offer</code></a>, <a href="../../java/util/Queue.html#poll--"><code>poll</code></a>, and <a href="../../java/util/Queue.html#peek--"><code>peek</code></a>, respectively, but throw exceptions instead of indicating failure via <tt>false</tt> or <tt>null</tt> returns.
/// <p>A <tt>Queue</tt> implementation that extends this class must minimally define a method <a href="../../java/util/Queue.html#offer-E-"><code>Queue.offer(E)</code></a> which does not permit insertion of <tt>null</tt> elements, along with methods <a href="../../java/util/Queue.html#peek--"><code>Queue.peek()</code></a>, <a href="../../java/util/Queue.html#poll--"><code>Queue.poll()</code></a>, <a href="../../java/util/Collection.html#size--"><code>Collection.size()</code></a>, and <a href="../../java/util/Collection.html#iterator--"><code>Collection.iterator()</code></a>. Typically, additional methods will be overridden as well. If these requirements cannot be met, consider instead subclassing <a href="../../java/util/AbstractCollection.html" title="class in java.util"><code>AbstractCollection</code></a>.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractQueue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractQueue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractQueue<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractQueue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractQueue")?;
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
}

impl<'mc> JavaAbstractQueue<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "element", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn offer(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn poll(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "poll", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peek", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}

impl<'mc> std::string::ToString for JavaAbstractQueue<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractQueue.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaQueue<'mc>> for JavaAbstractQueue<'mc> {
    fn into(self) -> crate::util::JavaQueue<'mc> {
        crate::util::JavaQueue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractQueue into crate::util::JavaQueue")
    }
}
impl<'mc> Into<crate::util::JavaAbstractCollection<'mc>> for JavaAbstractQueue<'mc> {
    fn into(self) -> crate::util::JavaAbstractCollection<'mc> {
        crate::util::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractQueue into crate::util::JavaAbstractCollection")
    }
}
/// An Entry maintaining a key and a value. The value may be changed using the <tt>setValue</tt> method. This class facilitates the process of building custom map implementations. For example, it may be convenient to return arrays of <tt>SimpleEntry</tt> instances in method <tt>Map.entrySet().toArray</tt>.
pub struct JavaAbstractMapSimpleEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractMapSimpleEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractMapSimpleEntry<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractMapSimpleEntry from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractMap$SimpleEntry")?;
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
}

impl<'mc> JavaAbstractMapSimpleEntry<'mc> {
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn key(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn set_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaAbstractMapSimpleEntry<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractMapSimpleEntry.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMapEntry<'mc>> for JavaAbstractMapSimpleEntry<'mc> {
    fn into(self) -> crate::util::JavaMapEntry<'mc> {
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractMapSimpleEntry into crate::util::JavaMapEntry")
    }
}
/// A class can implement the <code>Observer</code> interface when it wants to be informed of changes in observable objects.
///
/// This is a representation of an abstract class.
pub struct JavaObserver<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaObserver<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaObserver<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaObserver from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Observer")?;
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
}

impl<'mc> JavaObserver<'mc> {}
/// A tagging interface that all event listener interfaces must extend.
///
/// This is a representation of an abstract class.
pub struct JavaEventListener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaEventListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaEventListener<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaEventListener from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/EventListener")?;
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

impl<'mc> JavaEventListener<'mc> {}
/// A collection designed for holding elements prior to processing. Besides basic <a href="../../java/util/Collection.html" title="interface in java.util"><code>Collection</code></a> operations, queues provide additional insertion, extraction, and inspection operations. Each of these methods exists in two forms: one throws an exception if the operation fails, the other returns a special value (either <code>null</code> or <code>false</code>, depending on the operation). The latter form of the insert operation is designed specifically for use with capacity-restricted <code>Queue</code> implementations; in most implementations, insert operations cannot fail.
/// <table border="" cellpadding="3" cellspacing="1">
/// <caption>
/// Summary of Queue methods
/// </caption>
/// <tbody>
/// <tr>
/// <td></td>
/// <td align="CENTER"><em>Throws exception</em></td>
/// <td align="CENTER"><em>Returns special value</em></td>
/// </tr>
/// <tr>
/// <td><b>Insert</b></td>
/// <td><a href="../../java/util/Queue.html#add-E-"><code>add(e)</code></a></td>
/// <td><a href="../../java/util/Queue.html#offer-E-"><code>offer(e)</code></a></td>
/// </tr>
/// <tr>
/// <td><b>Remove</b></td>
/// <td><a href="../../java/util/Queue.html#remove--"><code>remove()</code></a></td>
/// <td><a href="../../java/util/Queue.html#poll--"><code>poll()</code></a></td>
/// </tr>
/// <tr>
/// <td><b>Examine</b></td>
/// <td><a href="../../java/util/Queue.html#element--"><code>element()</code></a></td>
/// <td><a href="../../java/util/Queue.html#peek--"><code>peek()</code></a></td>
/// </tr>
/// </tbody>
/// </table>
/// <p>Queues typically, but do not necessarily, order elements in a FIFO (first-in-first-out) manner. Among the exceptions are priority queues, which order elements according to a supplied comparator, or the elements' natural ordering, and LIFO queues (or stacks) which order the elements LIFO (last-in-first-out). Whatever the ordering used, the <em>head</em> of the queue is that element which would be removed by a call to <a href="../../java/util/Queue.html#remove--"><code>remove()</code></a> or <a href="../../java/util/Queue.html#poll--"><code>poll()</code></a>. In a FIFO queue, all new elements are inserted at the <em>tail</em> of the queue. Other kinds of queues may use different placement rules. Every <code>Queue</code> implementation must specify its ordering properties.</p>
/// <p>The <a href="../../java/util/Queue.html#offer-E-"><code>offer</code></a> method inserts an element if possible, otherwise returning <code>false</code>. This differs from the <a href="../../java/util/Collection.html#add-E-"><code>Collection.add</code></a> method, which can fail to add an element only by throwing an unchecked exception. The <code>offer</code> method is designed for use when failure is a normal, rather than exceptional occurrence, for example, in fixed-capacity (or "bounded") queues.</p>
/// <p>The <a href="../../java/util/Queue.html#remove--"><code>remove()</code></a> and <a href="../../java/util/Queue.html#poll--"><code>poll()</code></a> methods remove and return the head of the queue. Exactly which element is removed from the queue is a function of the queue's ordering policy, which differs from implementation to implementation. The <code>remove()</code> and <code>poll()</code> methods differ only in their behavior when the queue is empty: the <code>remove()</code> method throws an exception, while the <code>poll()</code> method returns <code>null</code>.</p>
/// <p>The <a href="../../java/util/Queue.html#element--"><code>element()</code></a> and <a href="../../java/util/Queue.html#peek--"><code>peek()</code></a> methods return, but do not remove, the head of the queue.</p>
/// <p>The <code>Queue</code> interface does not define the <i>blocking queue methods</i>, which are common in concurrent programming. These methods, which wait for elements to appear or for space to become available, are defined in the <a title="interface in java.util.concurrent" href="../../java/util/concurrent/BlockingQueue.html"><code>BlockingQueue</code></a> interface, which extends this interface.</p>
/// <p><code>Queue</code> implementations generally do not allow insertion of <code>null</code> elements, although some implementations, such as <a title="class in java.util" href="../../java/util/LinkedList.html"><code>LinkedList</code></a>, do not prohibit insertion of <code>null</code>. Even in the implementations that permit it, <code>null</code> should not be inserted into a <code>Queue</code>, as <code>null</code> is also used as a special return value by the <code>poll</code> method to indicate that the queue contains no elements.</p>
/// <p><code>Queue</code> implementations generally do not define element-based versions of methods <code>equals</code> and <code>hashCode</code> but instead inherit the identity based versions from class <code>Object</code>, because element-based equality is not always well-defined for queues with the same elements but different ordering properties.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaQueue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaQueue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaQueue<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaQueue from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Queue")?;
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
}

impl<'mc> JavaQueue<'mc> {
    //

    pub fn offer(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn poll(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "poll", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peek", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "element", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::util::JavaCollection<'mc>> for JavaQueue<'mc> {
    fn into(self) -> crate::util::JavaCollection<'mc> {
        crate::util::JavaCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaQueue into crate::util::JavaCollection")
    }
}
/// An object that implements the Enumeration interface generates a series of elements, one at a time. Successive calls to the <code>nextElement</code> method return successive elements of the series.
/// <p>For example, to print all elements of a <tt>Vector&lt;E&gt;</tt> <i>v</i>:</p>
/// <pre> for (Enumeration&lt;E&gt; e = v.elements(); e.hasMoreElements();)
/// System.out.println(e.nextElement());</pre>
/// <p>Methods are provided to enumerate through the elements of a vector, the keys of a hashtable, and the values in a hashtable. Enumerations are also used to specify the input streams to a <code>SequenceInputStream</code>.</p>
/// <p>NOTE: The functionality of this interface is duplicated by the Iterator interface. In addition, Iterator adds an optional remove operation, and has shorter method names. New implementations should consider using Iterator in preference to Enumeration.</p>
///
/// This is a representation of an abstract class.
pub struct JavaEnumeration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaEnumeration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaEnumeration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaEnumeration from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Enumeration")?;
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
}

impl<'mc> JavaEnumeration<'mc> {
    //

    pub fn as_iterator(
        &self,
    ) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "asIterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn has_more_elements(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasMoreElements", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextElement", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// This class implements a hash table, which maps keys to values. Any non-<code>null</code> object can be used as a key or as a value.
/// <p>To successfully store and retrieve objects from a hashtable, the objects used as keys must implement the <code>hashCode</code> method and the <code>equals</code> method.</p>
/// <p>An instance of <code>Hashtable</code> has two parameters that affect its performance: <i>initial capacity</i> and <i>load factor</i>. The <i>capacity</i> is the number of <i>buckets</i> in the hash table, and the <i>initial capacity</i> is simply the capacity at the time the hash table is created. Note that the hash table is <i>open</i>: in the case of a "hash collision", a single bucket stores multiple entries, which must be searched sequentially. The <i>load factor</i> is a measure of how full the hash table is allowed to get before its capacity is automatically increased. The initial capacity and load factor parameters are merely hints to the implementation. The exact details as to when and whether the rehash method is invoked are implementation-dependent.</p>
/// <p>Generally, the default load factor (.75) offers a good tradeoff between time and space costs. Higher values decrease the space overhead but increase the time cost to look up an entry (which is reflected in most <tt>Hashtable</tt> operations, including <tt>get</tt> and <tt>put</tt>).</p>
/// <p>The initial capacity controls a tradeoff between wasted space and the need for <code>rehash</code> operations, which are time-consuming. No <code>rehash</code> operations will <i>ever</i> occur if the initial capacity is greater than the maximum number of entries the <tt>Hashtable</tt> will contain divided by its load factor. However, setting the initial capacity too high can waste space.</p>
/// <p>If many entries are to be made into a <code>Hashtable</code>, creating it with a sufficiently large capacity may allow the entries to be inserted more efficiently than letting it perform automatic rehashing as needed to grow the table.</p>
/// <p>This example creates a hashtable of numbers. It uses the names of the numbers as keys:</p>
/// <pre> <code>
/// Hashtable&lt;String, Integer&gt; numbers
/// = new Hashtable&lt;String, Integer&gt;();
/// numbers.put("one", 1);
/// numbers.put("two", 2);
/// numbers.put("three", 3);</code></pre>
/// <p>To retrieve a number, use the following code:</p>
/// <pre> <code>
/// Integer n = numbers.get("two");
/// if (n != null) {
/// System.out.println("two = " + n);
/// }</code></pre>
/// <p>The iterators returned by the <tt>iterator</tt> method of the collections returned by all of this class's "collection view methods" are <em>fail-fast</em>: if the Hashtable is structurally modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future. The Enumerations returned by Hashtable's keys and elements methods are <em>not</em> fail-fast.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>As of the Java 2 platform v1.2, this class was retrofitted to implement the <a href="../../java/util/Map.html" title="interface in java.util"><code>Map</code></a> interface, making it a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>. Unlike the new collection implementations, <code>Hashtable</code> is synchronized. If a thread-safe implementation is not needed, it is recommended to use <a href="../../java/util/HashMap.html" title="class in java.util"><code>HashMap</code></a> in place of <code>Hashtable</code>. If a thread-safe highly-concurrent implementation is desired, then it is recommended to use <a href="../../java/util/concurrent/ConcurrentHashMap.html" title="class in java.util.concurrent"><code>ConcurrentHashMap</code></a> in place of <code>Hashtable</code>.</p>
pub struct JavaHashtable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaHashtable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaHashtable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashtable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Hashtable")?;
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
}

impl<'mc> JavaHashtable<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn elements(
        &self,
    ) -> Result<crate::util::JavaEnumeration<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Enumeration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "elements", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaEnumeration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn keys(&self) -> Result<crate::util::JavaEnumeration<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Enumeration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keys", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaEnumeration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaHashtable<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaHashtable.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaHashtable<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaHashtable into crate::util::JavaMap")
    }
}
/// This class provides a skeletal implementation of the <tt>Set</tt> interface to minimize the effort required to implement this interface.
/// <p>The process of implementing a set by extending this class is identical to that of implementing a Collection by extending AbstractCollection, except that all of the methods and constructors in subclasses of this class must obey the additional constraints imposed by the <tt>Set</tt> interface (for instance, the add method must not permit addition of multiple instances of an object to a set).</p>
/// <p>Note that this class does not override any of the implementations from the <tt>AbstractCollection</tt> class. It merely adds implementations for <tt>equals</tt> and <tt>hashCode</tt>.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractSet from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractSet")?;
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
}

impl<'mc> JavaAbstractSet<'mc> {
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaAbstractSet<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractSet.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaSet<'mc>> for JavaAbstractSet<'mc> {
    fn into(self) -> crate::util::JavaSet<'mc> {
        crate::util::JavaSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractSet into crate::util::JavaSet")
    }
}
impl<'mc> Into<crate::util::JavaAbstractCollection<'mc>> for JavaAbstractSet<'mc> {
    fn into(self) -> crate::util::JavaAbstractCollection<'mc> {
        crate::util::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractSet into crate::util::JavaAbstractCollection")
    }
}
/// A container object which may or may not contain a <code>int</code> value. If a value is present, <code>isPresent()</code> will return <code>true</code> and <code>getAsInt()</code> will return the value.
/// <p>Additional methods that depend on the presence or absence of a contained value are provided, such as <a href="../../java/util/OptionalInt.html#orElse-int-"><code>orElse()</code></a> (return a default value if value not present) and <a href="../../java/util/OptionalInt.html#ifPresent-java.util.function.IntConsumer-"><code>ifPresent()</code></a> (execute a block of code if the value is present).</p>
/// <p>This is a <a href="../lang/doc-files/ValueBased.html">value-based</a> class; use of identity-sensitive operations (including reference equality (<code>==</code>), identity hash code, or synchronization) on instances of <code>OptionalInt</code> may have unpredictable results and should be avoided.</p>
pub struct JavaOptionalInt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaOptionalInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaOptionalInt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalInt from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/OptionalInt")?;
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
}

impl<'mc> JavaOptionalInt<'mc> {
    //

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn or_else_get(
        &self,
        arg0: impl Into<crate::util::function::JavaIntSupplier<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/IntSupplier;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn if_present(
        &self,
        arg0: impl Into<crate::util::function::JavaIntConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/IntConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::util::JavaOptionalInt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/util/OptionalInt;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let cls = jni.find_class("java/util/OptionalInt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalInt::from_raw(&jni, obj)
    }
    //

    pub fn empty(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaOptionalInt<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/OptionalInt;");
        let cls = jni.find_class("java/util/OptionalInt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "empty", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalInt::from_raw(&jni, obj)
    }
    //

    pub fn is_present(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn or_else(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn or_else_throw_with_supplier(
        &self,
        arg0: std::option::Option<impl Into<crate::util::function::JavaSupplier<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/function/Supplier;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "orElseThrow", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaOptionalInt<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaOptionalInt.toString: {}", err),
        }
    }
}

/// A Red-Black tree based <a href="../../java/util/NavigableMap.html" title="interface in java.util"><code>NavigableMap</code></a> implementation. The map is sorted according to the <a href="../../java/lang/Comparable.html" title="interface in java.lang">natural ordering</a> of its keys, or by a <a href="../../java/util/Comparator.html" title="interface in java.util"><code>Comparator</code></a> provided at map creation time, depending on which constructor is used.
/// <p>This implementation provides guaranteed log(n) time cost for the <code>containsKey</code>, <code>get</code>, <code>put</code> and <code>remove</code> operations. Algorithms are adaptations of those in Cormen, Leiserson, and Rivest's <em>Introduction to Algorithms</em>.</p>
/// <p>Note that the ordering maintained by a tree map, like any sorted map, and whether or not an explicit comparator is provided, must be <em>consistent with <code>equals</code></em> if this sorted map is to correctly implement the <code>Map</code> interface. (See <code>Comparable</code> or <code>Comparator</code> for a precise definition of <em>consistent with equals</em>.) This is so because the <code>Map</code> interface is defined in terms of the <code>equals</code> operation, but a sorted map performs all key comparisons using its <code>compareTo</code> (or <code>compare</code>) method, so two keys that are deemed equal by this method are, from the standpoint of the sorted map, equal. The behavior of a sorted map <em>is</em> well-defined even if its ordering is inconsistent with <code>equals</code>; it just fails to obey the general contract of the <code>Map</code> interface.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a map concurrently, and at least one of the threads modifies the map structurally, it <em>must</em> be synchronized externally. (A structural modification is any operation that adds or deletes one or more mappings; merely changing the value associated with an existing key is not a structural modification.) This is typically accomplished by synchronizing on some object that naturally encapsulates the map. If no such object exists, the map should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedSortedMap-java.util.SortedMap-"><code>Collections.synchronizedSortedMap</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the map:</p>
/// <pre> SortedMap m = Collections.synchronizedSortedMap(new TreeMap(...));</pre>
/// <p>The iterators returned by the <code>iterator</code> method of the collections returned by all of this class's "collection view methods" are <em>fail-fast</em>: if the map is structurally modified at any time after the iterator is created, in any way except through the iterator's own <code>remove</code> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <em>the fail-fast behavior of iterators should be used only to detect bugs.</em></p>
/// <p>All <code>Map.Entry</code> pairs returned by methods in this class and its views represent snapshots of mappings at the time they were produced. They do <strong>not</strong> support the <code>Entry.setValue</code> method. (Note however that it is possible to change mappings in the associated map using <code>put</code>.)</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaTreeMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaTreeMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaTreeMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaTreeMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/TreeMap")?;
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
}

impl<'mc> JavaTreeMap<'mc> {
    //

    pub fn last_key(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lastKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn lower_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lowerKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn floor_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floorKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ceiling_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceilingKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn higher_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higherKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll_first_entry(
        &self,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map$Entry;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollFirstEntry", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn poll_last_entry(
        &self,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map$Entry;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "pollLastEntry", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn first_entry(
        &self,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map$Entry;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "firstEntry", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn last_entry(&self) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map$Entry;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lastEntry", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn lower_entry(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lowerEntry",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn floor_entry(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "floorEntry",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ceiling_entry(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ceilingEntry",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn higher_entry(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "higherEntry",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaMapEntry::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn first_key(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "firstKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn comparator(
        &self,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Comparator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "comparator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}

impl<'mc> std::string::ToString for JavaTreeMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaTreeMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaAbstractMap<'mc>> for JavaTreeMap<'mc> {
    fn into(self) -> crate::util::JavaAbstractMap<'mc> {
        crate::util::JavaAbstractMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaTreeMap into crate::util::JavaAbstractMap")
    }
}
/// A linear collection that supports element insertion and removal at both ends. The name <i>deque</i> is short for "double ended queue" and is usually pronounced "deck". Most <code>Deque</code> implementations place no fixed limits on the number of elements they may contain, but this interface supports capacity-restricted deques as well as those with no fixed size limit.
/// <p>This interface defines methods to access the elements at both ends of the deque. Methods are provided to insert, remove, and examine the element. Each of these methods exists in two forms: one throws an exception if the operation fails, the other returns a special value (either <code>null</code> or <code>false</code>, depending on the operation). The latter form of the insert operation is designed specifically for use with capacity-restricted <code>Deque</code> implementations; in most implementations, insert operations cannot fail.</p>
/// <p>The twelve methods described above are summarized in the following table:</p>
/// <table border="" cellpadding="3" cellspacing="1">
/// <caption>
/// Summary of Deque methods
/// </caption>
/// <tbody>
/// <tr>
/// <td></td>
/// <td align="CENTER" colspan="2"><b>First Element (Head)</b></td>
/// <td align="CENTER" colspan="2"><b>Last Element (Tail)</b></td>
/// </tr>
/// <tr>
/// <td></td>
/// <td align="CENTER"><em>Throws exception</em></td>
/// <td align="CENTER"><em>Special value</em></td>
/// <td align="CENTER"><em>Throws exception</em></td>
/// <td align="CENTER"><em>Special value</em></td>
/// </tr>
/// <tr>
/// <td><b>Insert</b></td>
/// <td><a href="../../java/util/Deque.html#addFirst-E-"><code>addFirst(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#offerFirst-E-"><code>offerFirst(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#addLast-E-"><code>addLast(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#offerLast-E-"><code>offerLast(e)</code></a></td>
/// </tr>
/// <tr>
/// <td><b>Remove</b></td>
/// <td><a href="../../java/util/Deque.html#removeFirst--"><code>removeFirst()</code></a></td>
/// <td><a href="../../java/util/Deque.html#pollFirst--"><code>pollFirst()</code></a></td>
/// <td><a href="../../java/util/Deque.html#removeLast--"><code>removeLast()</code></a></td>
/// <td><a href="../../java/util/Deque.html#pollLast--"><code>pollLast()</code></a></td>
/// </tr>
/// <tr>
/// <td><b>Examine</b></td>
/// <td><a href="../../java/util/Deque.html#getFirst--"><code>getFirst()</code></a></td>
/// <td><a href="../../java/util/Deque.html#peekFirst--"><code>peekFirst()</code></a></td>
/// <td><a href="../../java/util/Deque.html#getLast--"><code>getLast()</code></a></td>
/// <td><a href="../../java/util/Deque.html#peekLast--"><code>peekLast()</code></a></td>
/// </tr>
/// </tbody>
/// </table>
/// <p>This interface extends the <a title="interface in java.util" href="../../java/util/Queue.html"><code>Queue</code></a> interface. When a deque is used as a queue, FIFO (First-In-First-Out) behavior results. Elements are added at the end of the deque and removed from the beginning. The methods inherited from the <code>Queue</code> interface are precisely equivalent to <code>Deque</code> methods as indicated in the following table:</p>
/// <table border="" cellpadding="3" cellspacing="1">
/// <caption>
/// Comparison of Queue and Deque methods
/// </caption>
/// <tbody>
/// <tr>
/// <td align="CENTER"><b><code>Queue</code> Method</b></td>
/// <td align="CENTER"><b>Equivalent <code>Deque</code> Method</b></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#add-E-"><code>add(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#addLast-E-"><code>addLast(e)</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#offer-E-"><code>offer(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#offerLast-E-"><code>offerLast(e)</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#remove--"><code>remove()</code></a></td>
/// <td><a href="../../java/util/Deque.html#removeFirst--"><code>removeFirst()</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#poll--"><code>poll()</code></a></td>
/// <td><a href="../../java/util/Deque.html#pollFirst--"><code>pollFirst()</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#element--"><code>element()</code></a></td>
/// <td><a href="../../java/util/Deque.html#getFirst--"><code>getFirst()</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Queue.html#peek--"><code>peek()</code></a></td>
/// <td><a href="../../java/util/Deque.html#peek--"><code>peekFirst()</code></a></td>
/// </tr>
/// </tbody>
/// </table>
/// <p>Deques can also be used as LIFO (Last-In-First-Out) stacks. This interface should be used in preference to the legacy <a title="class in java.util" href="../../java/util/Stack.html"><code>Stack</code></a> class. When a deque is used as a stack, elements are pushed and popped from the beginning of the deque. Stack methods are precisely equivalent to <code>Deque</code> methods as indicated in the table below:</p>
/// <table cellspacing="1" cellpadding="3" border="">
/// <caption>
/// Comparison of Stack and Deque methods
/// </caption>
/// <tbody>
/// <tr>
/// <td align="CENTER"><b>Stack Method</b></td>
/// <td align="CENTER"><b>Equivalent <code>Deque</code> Method</b></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Deque.html#push-E-"><code>push(e)</code></a></td>
/// <td><a href="../../java/util/Deque.html#addFirst-E-"><code>addFirst(e)</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Deque.html#pop--"><code>pop()</code></a></td>
/// <td><a href="../../java/util/Deque.html#removeFirst--"><code>removeFirst()</code></a></td>
/// </tr>
/// <tr>
/// <td><a href="../../java/util/Deque.html#peek--"><code>peek()</code></a></td>
/// <td><a href="../../java/util/Deque.html#peekFirst--"><code>peekFirst()</code></a></td>
/// </tr>
/// </tbody>
/// </table>
/// <p>Note that the <a href="../../java/util/Deque.html#peek--"><code>peek</code></a> method works equally well when a deque is used as a queue or a stack; in either case, elements are drawn from the beginning of the deque.</p>
/// <p>This interface provides two methods to remove interior elements, <a href="../../java/util/Deque.html#removeFirstOccurrence-java.lang.Object-"><code>removeFirstOccurrence</code></a> and <a href="../../java/util/Deque.html#removeLastOccurrence-java.lang.Object-"><code>removeLastOccurrence</code></a>.</p>
/// <p>Unlike the <a title="interface in java.util" href="../../java/util/List.html"><code>List</code></a> interface, this interface does not provide support for indexed access to elements.</p>
/// <p>While <code>Deque</code> implementations are not strictly required to prohibit the insertion of null elements, they are strongly encouraged to do so. Users of any <code>Deque</code> implementations that do allow null elements are strongly encouraged <i>not</i> to take advantage of the ability to insert nulls. This is so because <code>null</code> is used as a special return value by various methods to indicated that the deque is empty.</p>
/// <p><code>Deque</code> implementations generally do not define element-based versions of the <code>equals</code> and <code>hashCode</code> methods, but instead inherit the identity-based versions from class <code>Object</code>.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDeque<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDeque<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDeque<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDeque from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Deque")?;
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
}

impl<'mc> JavaDeque<'mc> {
    //

    pub fn push(&self, arg0: jni::objects::JObject<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn pop(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn add_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn poll_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn offer_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_first_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_last_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn descending_iterator(
        &self,
    ) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn poll(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "poll", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peek", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "element", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::util::JavaQueue<'mc>> for JavaDeque<'mc> {
    fn into(self) -> crate::util::JavaQueue<'mc> {
        crate::util::JavaQueue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaDeque into crate::util::JavaQueue")
    }
}
/// The <tt>Formattable</tt> interface must be implemented by any class that needs to perform custom formatting using the <tt>'s'</tt> conversion specifier of <a title="class in java.util" href="../../java/util/Formatter.html"><code>Formatter</code></a>. This interface allows basic control for formatting arbitrary objects. For example, the following class prints out different representations of a stock's name depending on the flags and length constraints: <code>import java.nio.CharBuffer; import java.util.Formatter; import java.util.Formattable; import java.util.Locale; import static java.util.FormattableFlags.*; ... public class StockName implements Formattable { private String symbol, companyName, frenchCompanyName; public StockName(String symbol, String companyName, String frenchCompanyName) { ... } ... public void formatTo(Formatter fmt, int f, int width, int precision) { StringBuilder sb = new StringBuilder(); // decide form of name String name = companyName; if (fmt.locale().equals(Locale.FRANCE)) name = frenchCompanyName; boolean alternate = (f &amp; ALTERNATE) == ALTERNATE; boolean usesymbol = alternate || (precision != -1 &amp;&amp; precision &lt; 10); String out = (usesymbol ? symbol : name); // apply precision if (precision == -1 || out.length() &lt; precision) { // write it all sb.append(out); } else { sb.append(out.substring(0, precision - 1)).append('*'); } // apply width and justification int len = sb.length(); if (len &lt; width) for (int i = 0; i &lt; width - len; i++) if ((f &amp; LEFT_JUSTIFY) == LEFT_JUSTIFY) sb.append(' '); else sb.insert(0, ' '); fmt.format(sb.toString()); } public String toString() { return String.format("%s - %s", symbol, companyName); } } </code>
/// <p>When used in conjunction with the <a href="../../java/util/Formatter.html" title="class in java.util"><code>Formatter</code></a>, the above class produces the following output for various format strings. <code>Formatter fmt = new Formatter(); StockName sn = new StockName("HUGE", "Huge Fruit, Inc.", "Fruit Titanesque, Inc."); fmt.format("%s", sn); // -&gt; "Huge Fruit, Inc." fmt.format("%s", sn.toString()); // -&gt; "HUGE - Huge Fruit, Inc." fmt.format("%#s", sn); // -&gt; "HUGE" fmt.format("%-10.8s", sn); // -&gt; "HUGE " fmt.format("%.12s", sn); // -&gt; "Huge Fruit,*" fmt.format(Locale.FRANCE, "%25s", sn); // -&gt; " Fruit Titanesque, Inc." </code></p>
/// <p>Formattables are not necessarily safe for multithreaded access. Thread safety is optional and may be enforced by classes that extend and implement this interface.</p>
/// <p>Unless otherwise specified, passing a <tt>null</tt> argument to any method in this interface will cause a <a href="../../java/lang/NullPointerException.html" title="class in java.lang"><code>NullPointerException</code></a> to be thrown.</p>
///
/// This is a representation of an abstract class.
pub struct JavaFormattable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaFormattable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaFormattable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaFormattable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Formattable")?;
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
}

impl<'mc> JavaFormattable<'mc> {}
/// A container object which may or may not contain a <code>double</code> value. If a value is present, <code>isPresent()</code> will return <code>true</code> and <code>getAsDouble()</code> will return the value.
/// <p>Additional methods that depend on the presence or absence of a contained value are provided, such as <a href="../../java/util/OptionalDouble.html#orElse-double-"><code>orElse()</code></a> (return a default value if value not present) and <a href="../../java/util/OptionalDouble.html#ifPresent-java.util.function.DoubleConsumer-"><code>ifPresent()</code></a> (execute a block of code if the value is present).</p>
/// <p>This is a <a href="../lang/doc-files/ValueBased.html">value-based</a> class; use of identity-sensitive operations (including reference equality (<code>==</code>), identity hash code, or synchronization) on instances of <code>OptionalDouble</code> may have unpredictable results and should be avoided.</p>
pub struct JavaOptionalDouble<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaOptionalDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaOptionalDouble<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalDouble from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/OptionalDouble")?;
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
}

impl<'mc> JavaOptionalDouble<'mc> {
    //

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAsDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn or_else_get(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleSupplier<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/DoubleSupplier;)D");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn if_present(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/DoubleConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<crate::util::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(D)Ljava/util/OptionalDouble;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = jni.find_class("java/util/OptionalDouble");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalDouble::from_raw(&jni, obj)
    }
    //

    pub fn empty(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaOptionalDouble<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/OptionalDouble;");
        let cls = jni.find_class("java/util/OptionalDouble");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "empty", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalDouble::from_raw(&jni, obj)
    }
    //

    pub fn is_present(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn or_else(&self, arg0: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(D)D");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn or_else_throw_with_supplier(
        &self,
        arg0: std::option::Option<impl Into<crate::util::function::JavaSupplier<'mc>>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/function/Supplier;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "orElseThrow", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaOptionalDouble<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaOptionalDouble.toString: {}", err),
        }
    }
}

/// A container object which may or may not contain a <code>long</code> value. If a value is present, <code>isPresent()</code> will return <code>true</code> and <code>getAsLong()</code> will return the value.
/// <p>Additional methods that depend on the presence or absence of a contained value are provided, such as <a href="../../java/util/OptionalLong.html#orElse-long-"><code>orElse()</code></a> (return a default value if value not present) and <a href="../../java/util/OptionalLong.html#ifPresent-java.util.function.LongConsumer-"><code>ifPresent()</code></a> (execute a block of code if the value is present).</p>
/// <p>This is a <a href="../lang/doc-files/ValueBased.html">value-based</a> class; use of identity-sensitive operations (including reference equality (<code>==</code>), identity hash code, or synchronization) on instances of <code>OptionalLong</code> may have unpredictable results and should be avoided.</p>
pub struct JavaOptionalLong<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaOptionalLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaOptionalLong<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaOptionalLong from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/OptionalLong")?;
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
}

impl<'mc> JavaOptionalLong<'mc> {
    //

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn or_else_get(
        &self,
        arg0: impl Into<crate::util::function::JavaLongSupplier<'mc>>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/LongSupplier;)J");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn if_present(
        &self,
        arg0: impl Into<crate::util::function::JavaLongConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/LongConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<crate::util::JavaOptionalLong<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/util/OptionalLong;");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let cls = jni.find_class("java/util/OptionalLong");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalLong::from_raw(&jni, obj)
    }
    //

    pub fn empty(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaOptionalLong<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/OptionalLong;");
        let cls = jni.find_class("java/util/OptionalLong");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "empty", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptionalLong::from_raw(&jni, obj)
    }
    //

    pub fn is_present(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn or_else(&self, arg0: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn or_else_throw_with_supplier(
        &self,
        arg0: std::option::Option<impl Into<crate::util::function::JavaSupplier<'mc>>>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/function/Supplier;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "orElseThrow", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaOptionalLong<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaOptionalLong.toString: {}", err),
        }
    }
}

/// <p>Hash table and linked list implementation of the <tt>Set</tt> interface, with predictable iteration order. This implementation differs from <tt>HashSet</tt> in that it maintains a doubly-linked list running through all of its entries. This linked list defines the iteration ordering, which is the order in which elements were inserted into the set (<i>insertion-order</i>). Note that insertion order is <i>not</i> affected if an element is <i>re-inserted</i> into the set. (An element <tt>e</tt> is reinserted into a set <tt>s</tt> if <tt>s.add(e)</tt> is invoked when <tt>s.contains(e)</tt> would return <tt>true</tt> immediately prior to the invocation.)</p>
/// <p>This implementation spares its clients from the unspecified, generally chaotic ordering provided by <a href="../../java/util/HashSet.html" title="class in java.util"><code>HashSet</code></a>, without incurring the increased cost associated with <a title="class in java.util" href="../../java/util/TreeSet.html"><code>TreeSet</code></a>. It can be used to produce a copy of a set that has the same order as the original, regardless of the original set's implementation:</p>
/// <pre> void foo(Set s) {
/// Set copy = new LinkedHashSet(s);
/// ...
/// }
/// </pre> This technique is particularly useful if a module takes a set on input, copies it, and later returns results whose order is determined by that of the copy. (Clients generally appreciate having things returned in the same order they were presented.)
/// <p>This class provides all of the optional <tt>Set</tt> operations, and permits null elements. Like <tt>HashSet</tt>, it provides constant-time performance for the basic operations (<tt>add</tt>, <tt>contains</tt> and <tt>remove</tt>), assuming the hash function disperses elements properly among the buckets. Performance is likely to be just slightly below that of <tt>HashSet</tt>, due to the added expense of maintaining the linked list, with one exception: Iteration over a <tt>LinkedHashSet</tt> requires time proportional to the <i>size</i> of the set, regardless of its capacity. Iteration over a <tt>HashSet</tt> is likely to be more expensive, requiring time proportional to its <i>capacity</i>.</p>
/// <p>A linked hash set has two parameters that affect its performance: <i>initial capacity</i> and <i>load factor</i>. They are defined precisely as for <tt>HashSet</tt>. Note, however, that the penalty for choosing an excessively high value for initial capacity is less severe for this class than for <tt>HashSet</tt>, as iteration times for this class are unaffected by capacity.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a linked hash set concurrently, and at least one of the threads modifies the set, it <em>must</em> be synchronized externally. This is typically accomplished by synchronizing on some object that naturally encapsulates the set. If no such object exists, the set should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedSet-java.util.Set-"><code>Collections.synchronizedSet</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the set:</p>
/// <pre> Set s = Collections.synchronizedSet(new LinkedHashSet(...));</pre>
/// <p>The iterators returned by this class's <tt>iterator</tt> method are <em>fail-fast</em>: if the set is modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaLinkedHashSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLinkedHashSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLinkedHashSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedHashSet from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/LinkedHashSet")?;
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
}

impl<'mc> JavaLinkedHashSet<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLinkedHashSet<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLinkedHashSet.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaSet<'mc>> for JavaLinkedHashSet<'mc> {
    fn into(self) -> crate::util::JavaSet<'mc> {
        crate::util::JavaSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedHashSet into crate::util::JavaSet")
    }
}
impl<'mc> Into<crate::util::JavaHashSet<'mc>> for JavaLinkedHashSet<'mc> {
    fn into(self) -> crate::util::JavaHashSet<'mc> {
        crate::util::JavaHashSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedHashSet into crate::util::JavaHashSet")
    }
}
/// A <code>Locale</code> object represents a specific geographical, political, or cultural region. An operation that requires a <code>Locale</code> to perform its task is called <em>locale-sensitive</em> and uses the <code>Locale</code> to tailor information for the user. For example, displaying a number is a locale-sensitive operation— the number should be formatted according to the customs and conventions of the user's native country, region, or culture.
/// <p>The <code>Locale</code> class implements IETF BCP 47 which is composed of <a href="http://tools.ietf.org/html/rfc4647">RFC 4647 "Matching of Language Tags"</a> and <a href="http://tools.ietf.org/html/rfc5646">RFC 5646 "Tags for Identifying Languages"</a> with support for the LDML (UTS#35, "Unicode Locale Data Markup Language") BCP 47-compatible extensions for locale data exchange.</p>
/// <p>A <code>Locale</code> object logically consists of the fields described below.</p>
/// <dl>
/// <dt>
/// <a name="def_language"><b>language</b></a>
/// </dt>
/// <dd>
/// ISO 639 alpha-2 or alpha-3 language code, or registered language subtags up to 8 alpha letters (for future enhancements). When a language has both an alpha-2 code and an alpha-3 code, the alpha-2 code must be used. You can find a full list of valid language codes in the IANA Language Subtag Registry (search for "Type: language"). The language field is case insensitive, but <code>Locale</code> always canonicalizes to lower case.
/// </dd>
/// <dd>
/// Well-formed language values have the form <code>[a-zA-Z]{2,8}</code>. Note that this is not the the full BCP47 language production, since it excludes extlang. They are not needed since modern three-letter language codes replace them.
/// </dd>
/// <dd>
/// Example: "en" (English), "ja" (Japanese), "kok" (Konkani)
/// </dd>
/// <dt>
/// <a name="def_script"><b>script</b></a>
/// </dt>
/// <dd>
/// ISO 15924 alpha-4 script code. You can find a full list of valid script codes in the IANA Language Subtag Registry (search for "Type: script"). The script field is case insensitive, but <code>Locale</code> always canonicalizes to title case (the first letter is upper case and the rest of the letters are lower case).
/// </dd>
/// <dd>
/// Well-formed script values have the form <code>[a-zA-Z]{4}</code>
/// </dd>
/// <dd>
/// Example: "Latn" (Latin), "Cyrl" (Cyrillic)
/// </dd>
/// <dt>
/// <a name="def_region"><b>country (region)</b></a>
/// </dt>
/// <dd>
/// ISO 3166 alpha-2 country code or UN M.49 numeric-3 area code. You can find a full list of valid country and region codes in the IANA Language Subtag Registry (search for "Type: region"). The country (region) field is case insensitive, but <code>Locale</code> always canonicalizes to upper case.
/// </dd>
/// <dd>
/// Well-formed country/region values have the form <code>[a-zA-Z]{2} | [0-9]{3}</code>
/// </dd>
/// <dd>
/// Example: "US" (United States), "FR" (France), "029" (Caribbean)
/// </dd>
/// <dt>
/// <a name="def_variant"><b>variant</b></a>
/// </dt>
/// <dd>
/// Any arbitrary value used to indicate a variation of a <code>Locale</code>. Where there are two or more variant values each indicating its own semantics, these values should be ordered by importance, with most important first, separated by underscore('_'). The variant field is case sensitive.
/// </dd>
/// <dd>
/// Note: IETF BCP 47 places syntactic restrictions on variant subtags. Also BCP 47 subtags are strictly used to indicate additional variations that define a language or its dialects that are not covered by any combinations of language, script and region subtags. You can find a full list of valid variant codes in the IANA Language Subtag Registry (search for "Type: variant").
/// <p>However, the variant field in <code>Locale</code> has historically been used for any kind of variation, not just language variations. For example, some supported variants available in Java SE Runtime Environments indicate alternative cultural behaviors such as calendar type or number script. In BCP 47 this kind of information, which does not identify the language, is supported by extension subtags or private use subtags.</p>
/// </dd>
/// <dd>
/// Well-formed variant values have the form <code>SUBTAG (('_'|'-') SUBTAG)*</code> where <code>SUBTAG = [0-9][0-9a-zA-Z]{3} | [0-9a-zA-Z]{5,8}</code>. (Note: BCP 47 only uses hyphen ('-') as a delimiter, this is more lenient).
/// </dd>
/// <dd>
/// Example: "polyton" (Polytonic Greek), "POSIX"
/// </dd>
/// <dt>
/// <a name="def_extensions"><b>extensions</b></a>
/// </dt>
/// <dd>
/// A map from single character keys to string values, indicating extensions apart from language identification. The extensions in <code>Locale</code> implement the semantics and syntax of BCP 47 extension subtags and private use subtags. The extensions are case insensitive, but <code>Locale</code> canonicalizes all extension keys and values to lower case. Note that extensions cannot have empty values.
/// </dd>
/// <dd>
/// Well-formed keys are single characters from the set <code>[0-9a-zA-Z]</code>. Well-formed values have the form <code>SUBTAG ('-' SUBTAG)*</code> where for the key 'x' <code>SUBTAG = [0-9a-zA-Z]{1,8}</code> and for other keys <code>SUBTAG = [0-9a-zA-Z]{2,8}</code> (that is, 'x' allows single-character subtags).
/// </dd>
/// <dd>
/// Example: key="u"/value="ca-japanese" (Japanese Calendar), key="x"/value="java-1-7"
/// </dd>
/// </dl><b>Note:</b> Although BCP 47 requires field values to be registered in the IANA Language Subtag Registry, the <code>Locale</code> class does not provide any validation features. The <code>Builder</code> only checks if an individual field satisfies the syntactic requirement (is well-formed), but does not validate the value itself. See <a title="class in java.util" href="../../java/util/Locale.Builder.html"><code>Locale.Builder</code></a> for details.
/// <h3><a name="def_locale_extension">Unicode locale/language extension</a></h3>
/// <p>UTS#35, "Unicode Locale Data Markup Language" defines optional attributes and keywords to override or refine the default behavior associated with a locale. A keyword is represented by a pair of key and type. For example, "nu-thai" indicates that Thai local digits (value:"thai") should be used for formatting numbers (key:"nu").</p>
/// <p>The keywords are mapped to a BCP 47 extension value using the extension key 'u' (<a href="../../java/util/Locale.html#UNICODE_LOCALE_EXTENSION"><code>UNICODE_LOCALE_EXTENSION</code></a>). The above example, "nu-thai", becomes the extension "u-nu-thai".code</p>
/// <p>Thus, when a <code>Locale</code> object contains Unicode locale attributes and keywords, <code>getExtension(UNICODE_LOCALE_EXTENSION)</code> will return a String representing this information, for example, "nu-thai". The <code>Locale</code> class also provides <a href="../../java/util/Locale.html#getUnicodeLocaleAttributes--"><code>getUnicodeLocaleAttributes()</code></a>, <a href="../../java/util/Locale.html#getUnicodeLocaleKeys--"><code>getUnicodeLocaleKeys()</code></a>, and <a href="../../java/util/Locale.html#getUnicodeLocaleType-java.lang.String-"><code>getUnicodeLocaleType(java.lang.String)</code></a> which allow you to access Unicode locale attributes and key/type pairs directly. When represented as a string, the Unicode Locale Extension lists attributes alphabetically, followed by key/type sequences with keys listed alphabetically (the order of subtags comprising a key's type is fixed when the type is defined)</p>
/// <p>A well-formed locale key has the form <code>[0-9a-zA-Z]{2}</code>. A well-formed locale type has the form <code>"" | [0-9a-zA-Z]{3,8} ('-' [0-9a-zA-Z]{3,8})*</code> (it can be empty, or a series of subtags 3-8 alphanums in length). A well-formed locale attribute has the form <code>[0-9a-zA-Z]{3,8}</code> (it is a single subtag with the same form as a locale type subtag).</p>
/// <p>The Unicode locale extension specifies optional behavior in locale-sensitive services. Although the LDML specification defines various keys and values, actual locale-sensitive service implementations in a Java Runtime Environment might not support any particular Unicode locale attributes or key/type pairs.</p>
/// <h4>Creating a Locale</h4>
/// <p>There are several different ways to create a <code>Locale</code> object.</p>
/// <h5>Builder</h5>
/// <p>Using <a title="class in java.util" href="../../java/util/Locale.Builder.html"><code>Locale.Builder</code></a> you can construct a <code>Locale</code> object that conforms to BCP 47 syntax.</p>
/// <h5>Constructors</h5>
/// <p>The <code>Locale</code> class provides three constructors:</p>
/// <blockquote>
/// <pre> <a href="../../java/util/Locale.html#Locale-java.lang.String-"><code>Locale(String language)</code></a>
/// <a href="../../java/util/Locale.html#Locale-java.lang.String-java.lang.String-"><code>Locale(String language, String country)</code></a>
/// <a href="../../java/util/Locale.html#Locale-java.lang.String-java.lang.String-java.lang.String-"><code>Locale(String language, String country, String variant)</code></a>
/// </pre>
/// </blockquote> These constructors allow you to create a <code>Locale</code> object with language, country and variant, but you cannot specify script or extensions.
/// <h5>Factory Methods</h5>
/// <p>The method <a href="../../java/util/Locale.html#forLanguageTag-java.lang.String-"><code>forLanguageTag(java.lang.String)</code></a> creates a <code>Locale</code> object for a well-formed BCP 47 language tag.</p>
/// <h5>Locale Constants</h5>
/// <p>The <code>Locale</code> class provides a number of convenient constants that you can use to create <code>Locale</code> objects for commonly used locales. For example, the following creates a <code>Locale</code> object for the United States:</p>
/// <blockquote>
/// <pre> Locale.US
/// </pre>
/// </blockquote>
/// <h4><a name="LocaleMatching">Locale Matching</a></h4>
/// <p>If an application or a system is internationalized and provides localized resources for multiple locales, it sometimes needs to find one or more locales (or language tags) which meet each user's specific preferences. Note that a term "language tag" is used interchangeably with "locale" in this locale matching documentation.</p>
/// <p>In order to do matching a user's preferred locales to a set of language tags, <a href="http://tools.ietf.org/html/rfc4647">RFC 4647 Matching of Language Tags</a> defines two mechanisms: filtering and lookup. <em>Filtering</em> is used to get all matching locales, whereas <em>lookup</em> is to choose the best matching locale. Matching is done case-insensitively. These matching mechanisms are described in the following sections.</p>
/// <p>A user's preference is called a <em>Language Priority List</em> and is expressed as a list of language ranges. There are syntactically two types of language ranges: basic and extended. See <a href="../../java/util/Locale.LanguageRange.html" title="class in java.util"><code>Locale.LanguageRange</code></a> for details.</p>
/// <h5>Filtering</h5>
/// <p>The filtering operation returns all matching language tags. It is defined in RFC 4647 as follows: "In filtering, each language range represents the least specific language tag (that is, the language tag with fewest number of subtags) that is an acceptable match. All of the language tags in the matching set of tags will have an equal or greater number of subtags than the language range. Every non-wildcard subtag in the language range will appear in every one of the matching language tags."</p>
/// <p>There are two types of filtering: filtering for basic language ranges (called "basic filtering") and filtering for extended language ranges (called "extended filtering"). They may return different results by what kind of language ranges are included in the given Language Priority List. <a title="enum in java.util" href="../../java/util/Locale.FilteringMode.html"><code>Locale.FilteringMode</code></a> is a parameter to specify how filtering should be done.</p>
/// <h5>Lookup</h5>
/// <p>The lookup operation returns the best matching language tags. It is defined in RFC 4647 as follows: "By contrast with filtering, each language range represents the most specific tag that is an acceptable match. The first matching tag found, according to the user's priority, is considered the closest match and is the item returned."</p>
/// <p>For example, if a Language Priority List consists of two language ranges, <code>"zh-Hant-TW"</code> and <code>"en-US"</code>, in prioritized order, lookup method progressively searches the language tags below in order to find the best matching language tag.</p>
/// <blockquote>
/// <pre>1. zh-Hant-TW
/// 2. zh-Hant
/// 3. zh
/// 4. en-US
/// 5. en
/// </pre>
/// </blockquote> If there is a language tag which matches completely to a language range above, the language tag is returned.
/// <p><code>"*"</code> is the special language range, and it is ignored in lookup.</p>
/// <p>If multiple language tags match as a result of the subtag <code>'*'</code> included in a language range, the first matching language tag returned by an <a title="interface in java.util" href="../../java/util/Iterator.html"><code>Iterator</code></a> over a <a title="interface in java.util" href="../../java/util/Collection.html"><code>Collection</code></a> of language tags is treated as the best matching one.</p>
/// <h4>Use of Locale</h4>
/// <p>Once you've created a <code>Locale</code> you can query it for information about itself. Use <code>getCountry</code> to get the country (or region) code and <code>getLanguage</code> to get the language code. You can use <code>getDisplayCountry</code> to get the name of the country suitable for displaying to the user. Similarly, you can use <code>getDisplayLanguage</code> to get the name of the language suitable for displaying to the user. Interestingly, the <code>getDisplayXXX</code> methods are themselves locale-sensitive and have two versions: one that uses the default <a href="../../java/util/Locale.Category.html#DISPLAY"><code>DISPLAY</code></a> locale and one that uses the locale specified as an argument.</p>
/// <p>The Java Platform provides a number of classes that perform locale-sensitive operations. For example, the <code>NumberFormat</code> class formats numbers, currency, and percentages in a locale-sensitive manner. Classes such as <code>NumberFormat</code> have several convenience methods for creating a default object of that type. For example, the <code>NumberFormat</code> class provides these three convenience methods for creating a default <code>NumberFormat</code> object:</p>
/// <blockquote>
/// <pre> NumberFormat.getInstance()
/// NumberFormat.getCurrencyInstance()
/// NumberFormat.getPercentInstance()
/// </pre>
/// </blockquote> Each of these methods has two variants; one with an explicit locale and one without; the latter uses the default <a href="../../java/util/Locale.Category.html#FORMAT"><code>FORMAT</code></a> locale:
/// <blockquote>
/// <pre> NumberFormat.getInstance(myLocale)
/// NumberFormat.getCurrencyInstance(myLocale)
/// NumberFormat.getPercentInstance(myLocale)
/// </pre>
/// </blockquote> A <code>Locale</code> is the mechanism for identifying the kind of object (<code>NumberFormat</code>) that you would like to get. The locale is <strong>just</strong> a mechanism for identifying objects, <strong>not</strong> a container for the objects themselves.
/// <h4>Compatibility</h4>
/// <p>In order to maintain compatibility with existing usage, Locale's constructors retain their behavior prior to the Java Runtime Environment version 1.7. The same is largely true for the <code>toString</code> method. Thus Locale objects can continue to be used as they were. In particular, clients who parse the output of toString into language, country, and variant fields can continue to do so (although this is strongly discouraged), although the variant field will have additional information in it if script or extensions are present.</p>
/// <p>In addition, BCP 47 imposes syntax restrictions that are not imposed by Locale's constructors. This means that conversions between some Locales and BCP 47 language tags cannot be made without losing information. Thus <code>toLanguageTag</code> cannot represent the state of locales whose language, country, or variant do not conform to BCP 47.</p>
/// <p>Because of these issues, it is recommended that clients migrate away from constructing non-conforming locales and use the <code>forLanguageTag</code> and <code>Locale.Builder</code> APIs instead. Clients desiring a string representation of the complete locale can then always rely on <code>toLanguageTag</code> for this purpose.</p>
/// <h5><a name="special_cases_constructor">Special cases</a></h5>
/// <p>For compatibility reasons, two non-conforming locales are treated as special cases. These are <b><tt>ja_JP_JP</tt></b> and <b><tt>th_TH_TH</tt></b>. These are ill-formed in BCP 47 since the variants are too short. To ease migration to BCP 47, these are treated specially during construction. These two cases (and only these) cause a constructor to generate an extension, all other values behave exactly as they did prior to Java 7.</p>
/// <p>Java has used <tt>ja_JP_JP</tt> to represent Japanese as used in Japan together with the Japanese Imperial calendar. This is now representable using a Unicode locale extension, by specifying the Unicode locale key <tt>ca</tt> (for "calendar") and type <tt>japanese</tt>. When the Locale constructor is called with the arguments "ja", "JP", "JP", the extension "u-ca-japanese" is automatically added.</p>
/// <p>Java has used <tt>th_TH_TH</tt> to represent Thai as used in Thailand together with Thai digits. This is also now representable using a Unicode locale extension, by specifying the Unicode locale key <tt>nu</tt> (for "number") and value <tt>thai</tt>. When the Locale constructor is called with the arguments "th", "TH", "TH", the extension "u-nu-thai" is automatically added.</p>
/// <h5>Serialization</h5>
/// <p>During serialization, writeObject writes all fields to the output stream, including extensions.</p>
/// <p>During deserialization, readResolve adds extensions as described in <a href="#special_cases_constructor">Special Cases</a>, only for the two cases th_TH_TH and ja_JP_JP.</p>
/// <h5>Legacy language codes</h5>
/// <p>Locale's constructor has always converted three language codes to their earlier, obsoleted forms: <tt>he</tt> maps to <tt>iw</tt>, <tt>yi</tt> maps to <tt>ji</tt>, and <tt>id</tt> maps to <tt>in</tt>. This continues to be the case, in order to not break backwards compatibility.</p>
/// <p>The APIs added in 1.7 map between the old and new language codes, maintaining the old codes internal to Locale (so that <code>getLanguage</code> and <code>toString</code> reflect the old code), but using the new codes in the BCP 47 language tag APIs (so that <code>toLanguageTag</code> reflects the new one). This preserves the equivalence between Locales no matter which code or API is used to construct them. Java's default resource bundle lookup mechanism also implements this mapping, so that resources can be named using either convention, see <a href="../../java/util/ResourceBundle.Control.html" title="class in java.util"><code>ResourceBundle.Control</code></a>.</p>
/// <h5>Three-letter language/country(region) codes</h5>
/// <p>The Locale constructors have always specified that the language and the country param be two characters in length, although in practice they have accepted any length. The specification has now been relaxed to allow language codes of two to eight characters and country (region) codes of two to three characters, and in particular, three-letter language codes and three-digit region codes as specified in the IANA Language Subtag Registry. For compatibility, the implementation still does not impose a length constraint.</p>
pub struct JavaLocale<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLocale<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLocale<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLocale from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Locale")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLocale object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLocale<'mc> {
    //

    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    pub fn script(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getScript", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn country(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCountry", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn variant(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVariant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn has_extensions(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasExtensions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn unicode_locale_attributes(
        &self,
    ) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUnicodeLocaleAttributes",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_unicode_locale_type(
        &self,
        arg0: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUnicodeLocaleType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn unicode_locale_keys(
        &self,
    ) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUnicodeLocaleKeys",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_display_language(
        &self,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Locale;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayLanguage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_display_script(
        &self,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Locale;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayScript",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_display_country(
        &self,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Locale;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayCountry",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn get_display_variant(
        &self,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Locale;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayVariant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn lookup_tag(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaList<'mc>>,
        arg1: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "lookupTag",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    //

    //

    pub fn strip_extensions(
        &self,
    ) -> Result<crate::util::JavaLocale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Locale;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "stripExtensions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocale::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_extension(&self, arg0: u16) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExtension",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn extension_keys(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExtensionKeys",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn to_language_tag(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toLanguageTag", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn for_language_tag(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/Locale");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "forLanguageTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaLocale::from_raw(&jni, obj)
    }
    //

    pub fn iso3language(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getISO3Language", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn iso3country(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getISO3Country", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn language(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLanguage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn lookup(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaList<'mc>>,
        arg1: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<crate::util::JavaLocale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Locale");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "lookup",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaLocale::from_raw(&jni, obj)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLocale<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLocale.toString: {}", err),
        }
    }
}

/// Hash table based implementation of the <tt>Map</tt> interface. This implementation provides all of the optional map operations, and permits <tt>null</tt> values and the <tt>null</tt> key. (The <tt>HashMap</tt> class is roughly equivalent to <tt>Hashtable</tt>, except that it is unsynchronized and permits nulls.) This class makes no guarantees as to the order of the map; in particular, it does not guarantee that the order will remain constant over time.
/// <p>This implementation provides constant-time performance for the basic operations (<tt>get</tt> and <tt>put</tt>), assuming the hash function disperses the elements properly among the buckets. Iteration over collection views requires time proportional to the "capacity" of the <tt>HashMap</tt> instance (the number of buckets) plus its size (the number of key-value mappings). Thus, it's very important not to set the initial capacity too high (or the load factor too low) if iteration performance is important.</p>
/// <p>An instance of <tt>HashMap</tt> has two parameters that affect its performance: <i>initial capacity</i> and <i>load factor</i>. The <i>capacity</i> is the number of buckets in the hash table, and the initial capacity is simply the capacity at the time the hash table is created. The <i>load factor</i> is a measure of how full the hash table is allowed to get before its capacity is automatically increased. When the number of entries in the hash table exceeds the product of the load factor and the current capacity, the hash table is <i>rehashed</i> (that is, internal data structures are rebuilt) so that the hash table has approximately twice the number of buckets.</p>
/// <p>As a general rule, the default load factor (.75) offers a good tradeoff between time and space costs. Higher values decrease the space overhead but increase the lookup cost (reflected in most of the operations of the <tt>HashMap</tt> class, including <tt>get</tt> and <tt>put</tt>). The expected number of entries in the map and its load factor should be taken into account when setting its initial capacity, so as to minimize the number of rehash operations. If the initial capacity is greater than the maximum number of entries divided by the load factor, no rehash operations will ever occur.</p>
/// <p>If many mappings are to be stored in a <tt>HashMap</tt> instance, creating it with a sufficiently large capacity will allow the mappings to be stored more efficiently than letting it perform automatic rehashing as needed to grow the table. Note that using many keys with the same <code>hashCode()</code> is a sure way to slow down performance of any hash table. To ameliorate impact, when keys are <a href="../../java/lang/Comparable.html" title="interface in java.lang"><code>Comparable</code></a>, this class may use comparison order among keys to help break ties.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a hash map concurrently, and at least one of the threads modifies the map structurally, it <i>must</i> be synchronized externally. (A structural modification is any operation that adds or deletes one or more mappings; merely changing the value associated with a key that an instance already contains is not a structural modification.) This is typically accomplished by synchronizing on some object that naturally encapsulates the map. If no such object exists, the map should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedMap-java.util.Map-"><code>Collections.synchronizedMap</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the map:</p>
/// <pre> Map m = Collections.synchronizedMap(new HashMap(...));</pre>
/// <p>The iterators returned by all of this class's "collection view methods" are <i>fail-fast</i>: if the map is structurally modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaHashMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/HashMap")?;
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
}

impl<'mc> JavaHashMap<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaHashMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaHashMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaHashMap<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaHashMap into crate::util::JavaMap")
    }
}
impl<'mc> Into<crate::util::JavaAbstractMap<'mc>> for JavaHashMap<'mc> {
    fn into(self) -> crate::util::JavaAbstractMap<'mc> {
        crate::util::JavaAbstractMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaHashMap into crate::util::JavaAbstractMap")
    }
}
/// This class implements the <tt>Map</tt> interface with a hash table, using reference-equality in place of object-equality when comparing keys (and values). In other words, in an <tt>IdentityHashMap</tt>, two keys <tt>k1</tt> and <tt>k2</tt> are considered equal if and only if <tt>(k1==k2)</tt>. (In normal <tt>Map</tt> implementations (like <tt>HashMap</tt>) two keys <tt>k1</tt> and <tt>k2</tt> are considered equal if and only if <tt>(k1==null ? k2==null : k1.equals(k2))</tt>.)
/// <p><b>This class is <i>not</i> a general-purpose <tt>Map</tt> implementation! While this class implements the <tt>Map</tt> interface, it intentionally violates <tt>Map's</tt> general contract, which mandates the use of the <tt>equals</tt> method when comparing objects. This class is designed for use only in the rare cases wherein reference-equality semantics are required.</b></p>
/// <p>A typical use of this class is <i>topology-preserving object graph transformations</i>, such as serialization or deep-copying. To perform such a transformation, a program must maintain a "node table" that keeps track of all the object references that have already been processed. The node table must not equate distinct objects even if they happen to be equal. Another typical use of this class is to maintain <i>proxy objects</i>. For example, a debugging facility might wish to maintain a proxy object for each object in the program being debugged.</p>
/// <p>This class provides all of the optional map operations, and permits <tt>null</tt> values and the <tt>null</tt> key. This class makes no guarantees as to the order of the map; in particular, it does not guarantee that the order will remain constant over time.</p>
/// <p>This class provides constant-time performance for the basic operations (<tt>get</tt> and <tt>put</tt>), assuming the system identity hash function (<a href="../../java/lang/System.html#identityHashCode-java.lang.Object-"><code>System.identityHashCode(Object)</code></a>) disperses elements properly among the buckets.</p>
/// <p>This class has one tuning parameter (which affects performance but not semantics): <i>expected maximum size</i>. This parameter is the maximum number of key-value mappings that the map is expected to hold. Internally, this parameter is used to determine the number of buckets initially comprising the hash table. The precise relationship between the expected maximum size and the number of buckets is unspecified.</p>
/// <p>If the size of the map (the number of key-value mappings) sufficiently exceeds the expected maximum size, the number of buckets is increased. Increasing the number of buckets ("rehashing") may be fairly expensive, so it pays to create identity hash maps with a sufficiently large expected maximum size. On the other hand, iteration over collection views requires time proportional to the number of buckets in the hash table, so it pays not to set the expected maximum size too high if you are especially concerned with iteration performance or memory usage.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access an identity hash map concurrently, and at least one of the threads modifies the map structurally, it <i>must</i> be synchronized externally. (A structural modification is any operation that adds or deletes one or more mappings; merely changing the value associated with a key that an instance already contains is not a structural modification.) This is typically accomplished by synchronizing on some object that naturally encapsulates the map. If no such object exists, the map should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedMap-java.util.Map-"><code>Collections.synchronizedMap</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the map:</p>
/// <pre> Map m = Collections.synchronizedMap(new IdentityHashMap(...));</pre>
/// <p>The iterators returned by the <tt>iterator</tt> method of the collections returned by all of this class's "collection view methods" are <i>fail-fast</i>: if the map is structurally modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a title="class in java.util" href="../../java/util/ConcurrentModificationException.html"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>fail-fast iterators should be used only to detect bugs.</i></p>
/// <p>Implementation note: This is a simple <i>linear-probe</i> hash table, as described for example in texts by Sedgewick and Knuth. The array alternates holding keys and values. (This has better locality for large tables than does using separate arrays.) For many JRE implementations and operation mixes, this class will yield better performance than <a href="../../java/util/HashMap.html" title="class in java.util"><code>HashMap</code></a> (which uses <i>chaining</i> rather than linear-probing).</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaIdentityHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIdentityHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIdentityHashMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIdentityHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/IdentityHashMap")?;
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
}

impl<'mc> JavaIdentityHashMap<'mc> {
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}

impl<'mc> std::string::ToString for JavaIdentityHashMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaIdentityHashMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaIdentityHashMap<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaIdentityHashMap into crate::util::JavaMap")
    }
}
impl<'mc> Into<crate::util::JavaAbstractMap<'mc>> for JavaIdentityHashMap<'mc> {
    fn into(self) -> crate::util::JavaAbstractMap<'mc> {
        crate::util::JavaAbstractMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaIdentityHashMap into crate::util::JavaAbstractMap")
    }
}
/// <p>Hash table and linked list implementation of the <tt>Map</tt> interface, with predictable iteration order. This implementation differs from <tt>HashMap</tt> in that it maintains a doubly-linked list running through all of its entries. This linked list defines the iteration ordering, which is normally the order in which keys were inserted into the map (<i>insertion-order</i>). Note that insertion order is not affected if a key is <i>re-inserted</i> into the map. (A key <tt>k</tt> is reinserted into a map <tt>m</tt> if <tt>m.put(k, v)</tt> is invoked when <tt>m.containsKey(k)</tt> would return <tt>true</tt> immediately prior to the invocation.)</p>
/// <p>This implementation spares its clients from the unspecified, generally chaotic ordering provided by <a title="class in java.util" href="../../java/util/HashMap.html"><code>HashMap</code></a> (and <a title="class in java.util" href="../../java/util/Hashtable.html"><code>Hashtable</code></a>), without incurring the increased cost associated with <a title="class in java.util" href="../../java/util/TreeMap.html"><code>TreeMap</code></a>. It can be used to produce a copy of a map that has the same order as the original, regardless of the original map's implementation:</p>
/// <pre> void foo(Map m) {
/// Map copy = new LinkedHashMap(m);
/// ...
/// }
/// </pre> This technique is particularly useful if a module takes a map on input, copies it, and later returns results whose order is determined by that of the copy. (Clients generally appreciate having things returned in the same order they were presented.)
/// <p>A special <a href="../../java/util/LinkedHashMap.html#LinkedHashMap-int-float-boolean-"><code>constructor</code></a> is provided to create a linked hash map whose order of iteration is the order in which its entries were last accessed, from least-recently accessed to most-recently (<i>access-order</i>). This kind of map is well-suited to building LRU caches. Invoking the <code>put</code>, <code>putIfAbsent</code>, <code>get</code>, <code>getOrDefault</code>, <code>compute</code>, <code>computeIfAbsent</code>, <code>computeIfPresent</code>, or <code>merge</code> methods results in an access to the corresponding entry (assuming it exists after the invocation completes). The <code>replace</code> methods only result in an access of the entry if the value is replaced. The <code>putAll</code> method generates one entry access for each mapping in the specified map, in the order that key-value mappings are provided by the specified map's entry set iterator. <i>No other methods generate entry accesses.</i> In particular, operations on collection-views do <i>not</i> affect the order of iteration of the backing map.</p>
/// <p>The <a href="../../java/util/LinkedHashMap.html#removeEldestEntry-java.util.Map.Entry-"><code>removeEldestEntry(Map.Entry)</code></a> method may be overridden to impose a policy for removing stale mappings automatically when new mappings are added to the map.</p>
/// <p>This class provides all of the optional <tt>Map</tt> operations, and permits null elements. Like <tt>HashMap</tt>, it provides constant-time performance for the basic operations (<tt>add</tt>, <tt>contains</tt> and <tt>remove</tt>), assuming the hash function disperses elements properly among the buckets. Performance is likely to be just slightly below that of <tt>HashMap</tt>, due to the added expense of maintaining the linked list, with one exception: Iteration over the collection-views of a <tt>LinkedHashMap</tt> requires time proportional to the <i>size</i> of the map, regardless of its capacity. Iteration over a <tt>HashMap</tt> is likely to be more expensive, requiring time proportional to its <i>capacity</i>.</p>
/// <p>A linked hash map has two parameters that affect its performance: <i>initial capacity</i> and <i>load factor</i>. They are defined precisely as for <tt>HashMap</tt>. Note, however, that the penalty for choosing an excessively high value for initial capacity is less severe for this class than for <tt>HashMap</tt>, as iteration times for this class are unaffected by capacity.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a linked hash map concurrently, and at least one of the threads modifies the map structurally, it <em>must</em> be synchronized externally. This is typically accomplished by synchronizing on some object that naturally encapsulates the map. If no such object exists, the map should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedMap-java.util.Map-"><code>Collections.synchronizedMap</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the map:</p>
/// <pre> Map m = Collections.synchronizedMap(new LinkedHashMap(...));</pre> A structural modification is any operation that adds or deletes one or more mappings or, in the case of access-ordered linked hash maps, affects iteration order. In insertion-ordered linked hash maps, merely changing the value associated with a key that is already contained in the map is not a structural modification. <strong>In access-ordered linked hash maps, merely querying the map with <tt>get</tt> is a structural modification. </strong>)
/// <p>The iterators returned by the <tt>iterator</tt> method of the collections returned by all of this class's collection view methods are <em>fail-fast</em>: if the map is structurally modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the iterator will throw a <a title="class in java.util" href="../../java/util/ConcurrentModificationException.html"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>The spliterators returned by the spliterator method of the collections returned by all of this class's collection view methods are <em><a href="Spliterator.html#binding">late-binding</a></em>, <em>fail-fast</em>, and additionally report <a href="../../java/util/Spliterator.html#ORDERED"><code>Spliterator.ORDERED</code></a>.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaLinkedHashMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLinkedHashMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLinkedHashMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedHashMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/LinkedHashMap")?;
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
}

impl<'mc> JavaLinkedHashMap<'mc> {
    //

    pub fn get(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn values(&self) -> Result<crate::util::JavaCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "values", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiFunction;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn entry_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "entrySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn key_set(&self) -> Result<crate::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "keySet", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_or_default(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOrDefault",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn put(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "put",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/Object;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "replace", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn merge(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn put_all(
        &self,
        arg0: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn put_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "putIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn contains_key(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsKey",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compute_if_absent(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfAbsent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compute_if_present(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<crate::util::function::JavaBiFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "computeIfPresent",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLinkedHashMap<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLinkedHashMap.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaMap<'mc>> for JavaLinkedHashMap<'mc> {
    fn into(self) -> crate::util::JavaMap<'mc> {
        crate::util::JavaMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedHashMap into crate::util::JavaMap")
    }
}
impl<'mc> Into<crate::util::JavaHashMap<'mc>> for JavaLinkedHashMap<'mc> {
    fn into(self) -> crate::util::JavaHashMap<'mc> {
        crate::util::JavaHashMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedHashMap into crate::util::JavaHashMap")
    }
}
/// This class provides a skeletal implementation of the <tt>List</tt> interface to minimize the effort required to implement this interface backed by a "sequential access" data store (such as a linked list). For random access data (such as an array), <tt>AbstractList</tt> should be used in preference to this class.
/// <p>This class is the opposite of the <tt>AbstractList</tt> class in the sense that it implements the "random access" methods (<tt>get(int index)</tt>, <tt>set(int index, E element)</tt>, <tt>add(int index, E element)</tt> and <tt>remove(int index)</tt>) on top of the list's list iterator, instead of the other way around.</p>
/// <p>To implement a list the programmer needs only to extend this class and provide implementations for the <tt>listIterator</tt> and <tt>size</tt> methods. For an unmodifiable list, the programmer need only implement the list iterator's <tt>hasNext</tt>, <tt>next</tt>, <tt>hasPrevious</tt>, <tt>previous</tt> and <tt>index</tt> methods.</p>
/// <p>For a modifiable list the programmer should additionally implement the list iterator's <tt>set</tt> method. For a variable-size list the programmer should additionally implement the list iterator's <tt>remove</tt> and <tt>add</tt> methods.</p>
/// <p>The programmer should generally provide a void (no argument) and collection constructor, as per the recommendation in the <tt>Collection</tt> interface specification.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractSequentialList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractSequentialList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractSequentialList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractSequentialList from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractSequentialList")?;
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
}

impl<'mc> JavaAbstractSequentialList<'mc> {
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaAbstractSequentialList<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractSequentialList.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaAbstractList<'mc>> for JavaAbstractSequentialList<'mc> {
    fn into(self) -> crate::util::JavaAbstractList<'mc> {
        crate::util::JavaAbstractList::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting JavaAbstractSequentialList into crate::util::JavaAbstractList",
        )
    }
}
/// A comparison function, which imposes a <i>total ordering</i> on some collection of objects. Comparators can be passed to a sort method (such as <a href="../../java/util/Collections.html#sort-java.util.List-java.util.Comparator-"><code>Collections.sort</code></a> or <a href="../../java/util/Arrays.html#sort-T:A-java.util.Comparator-"><code>Arrays.sort</code></a>) to allow precise control over the sort order. Comparators can also be used to control the order of certain data structures (such as <a title="interface in java.util" href="../../java/util/SortedSet.html"><code>sorted sets</code></a> or <a href="../../java/util/SortedMap.html" title="interface in java.util"><code>sorted maps</code></a>), or to provide an ordering for collections of objects that don't have a <a href="../../java/lang/Comparable.html" title="interface in java.lang"><code>natural ordering</code></a>.
/// <p>The ordering imposed by a comparator <tt>c</tt> on a set of elements <tt>S</tt> is said to be <i>consistent with equals</i> if and only if <tt>c.compare(e1, e2)==0</tt> has the same boolean value as <tt>e1.equals(e2)</tt> for every <tt>e1</tt> and <tt>e2</tt> in <tt>S</tt>.</p>
/// <p>Caution should be exercised when using a comparator capable of imposing an ordering inconsistent with equals to order a sorted set (or sorted map). Suppose a sorted set (or sorted map) with an explicit comparator <tt>c</tt> is used with elements (or keys) drawn from a set <tt>S</tt>. If the ordering imposed by <tt>c</tt> on <tt>S</tt> is inconsistent with equals, the sorted set (or sorted map) will behave "strangely." In particular the sorted set (or sorted map) will violate the general contract for set (or map), which is defined in terms of <tt>equals</tt>.</p>
/// <p>For example, suppose one adds two elements <code>a</code> and <code>b</code> such that <code>(a.equals(b) &amp;&amp; c.compare(a, b) != 0)</code> to an empty <code>TreeSet</code> with comparator <code>c</code>. The second <code>add</code> operation will return true (and the size of the tree set will increase) because <code>a</code> and <code>b</code> are not equivalent from the tree set's perspective, even though this is contrary to the specification of the <a href="../../java/util/Set.html#add-E-"><code>Set.add</code></a> method.</p>
/// <p>Note: It is generally a good idea for comparators to also implement <tt>java.io.Serializable</tt>, as they may be used as ordering methods in serializable data structures (like <a href="../../java/util/TreeSet.html" title="class in java.util"><code>TreeSet</code></a>, <a title="class in java.util" href="../../java/util/TreeMap.html"><code>TreeMap</code></a>). In order for the data structure to serialize successfully, the comparator (if provided) must implement <tt>Serializable</tt>.</p>
/// <p>For the mathematically inclined, the <i>relation</i> that defines the <i>imposed ordering</i> that a given comparator <tt>c</tt> imposes on a given set of objects <tt>S</tt> is:</p>
/// <pre> {(x, y) such that c.compare(x, y) &lt;= 0}.
/// </pre> The <i>quotient</i> for this total order is:
/// <pre> {(x, y) such that c.compare(x, y) == 0}.
/// </pre> It follows immediately from the contract for <tt>compare</tt> that the quotient is an <i>equivalence relation</i> on <tt>S</tt>, and that the imposed ordering is a <i>total order</i> on <tt>S</tt>. When we say that the ordering imposed by <tt>c</tt> on <tt>S</tt> is <i>consistent with equals</i>, we mean that the quotient for the ordering is the equivalence relation defined by the objects' <a href="../../java/lang/Object.html#equals-java.lang.Object-"><code>equals(Object)</code></a> method(s):
/// <pre> {(x, y) such that x.equals(y)}. </pre>
/// <p>Unlike <code>Comparable</code>, a comparator may optionally permit comparison of null arguments, while maintaining the requirements for an equivalence relation.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaComparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaComparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaComparator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaComparator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Comparator")?;
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
}

impl<'mc> JavaComparator<'mc> {
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compare(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn reverse_order(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Comparator;");
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "reverseOrder", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn comparing_with_function(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
        arg1: std::option::Option<impl Into<crate::util::JavaComparator<'mc>>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Function;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Comparator;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Ljava/util/Comparator;";
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "comparing", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn then_comparing_with_function(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
        arg1: std::option::Option<impl Into<crate::util::JavaComparator<'mc>>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Function;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Comparator;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Ljava/util/Comparator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "thenComparing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn comparing_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::function::JavaToIntFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "comparingInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn comparing_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::function::JavaToLongFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "comparingLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn comparing_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::function::JavaToDoubleFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "comparingDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn reversed(&self) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Comparator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "reversed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn then_comparing_int(
        &self,
        arg0: impl Into<crate::util::function::JavaToIntFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn then_comparing_long(
        &self,
        arg0: impl Into<crate::util::function::JavaToLongFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn then_comparing_double(
        &self,
        arg0: impl Into<crate::util::function::JavaToDoubleFunction<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "thenComparingDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaComparator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn natural_order(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Comparator;");
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "naturalOrder", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn nulls_first(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "nullsFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn nulls_last(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)Ljava/util/Comparator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "nullsLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
}
/// An instance of this class is used to generate a stream of pseudorandom numbers. The class uses a 48-bit seed, which is modified using a linear congruential formula. (See Donald Knuth, <i>The Art of Computer Programming, Volume 2</i>, Section 3.2.1.)
/// <p>If two instances of <code>Random</code> are created with the same seed, and the same sequence of method calls is made for each, they will generate and return identical sequences of numbers. In order to guarantee this property, particular algorithms are specified for the class <code>Random</code>. Java implementations must use all the algorithms shown here for the class <code>Random</code>, for the sake of absolute portability of Java code. However, subclasses of class <code>Random</code> are permitted to use other algorithms, so long as they adhere to the general contracts for all the methods.</p>
/// <p>The algorithms implemented by class <code>Random</code> use a <code>protected</code> utility method that on each invocation can supply up to 32 pseudorandomly generated bits.</p>
/// <p>Many applications will find the method <a href="../../java/lang/Math.html#random--"><code>Math.random()</code></a> simpler to use.</p>
/// <p>Instances of <code>java.util.Random</code> are threadsafe. However, the concurrent use of the same <code>java.util.Random</code> instance across threads may encounter contention and consequent poor performance. Consider instead using <a title="class in java.util.concurrent" href="../../java/util/concurrent/ThreadLocalRandom.html"><code>ThreadLocalRandom</code></a> in multithreaded designs.</p>
/// <p>Instances of <code>java.util.Random</code> are not cryptographically secure. Consider instead using <a href="../../java/security/SecureRandom.html" title="class in java.security"><code>SecureRandom</code></a> to get a cryptographically secure pseudo-random number generator for use by security-sensitive applications.</p>
pub struct JavaRandom<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaRandom<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaRandom<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaRandom from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Random")?;
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
}

impl<'mc> JavaRandom<'mc> {
    //

    pub fn next_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float_with_float(
        &self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_bytes(&self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double_with_double(
        &self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int_with_int(
        &self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_deprecated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}

impl<'mc> std::string::ToString for JavaRandom<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaRandom.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::random::JavaRandomGenerator<'mc>> for JavaRandom<'mc> {
    fn into(self) -> crate::util::random::JavaRandomGenerator<'mc> {
        crate::util::random::JavaRandomGenerator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaRandom into crate::util::random::JavaRandomGenerator")
    }
}
/// An ordered collection (also known as a <i>sequence</i>). The user of this interface has precise control over where in the list each element is inserted. The user can access elements by their integer index (position in the list), and search for elements in the list.
/// <p>Unlike sets, lists typically allow duplicate elements. More formally, lists typically allow pairs of elements <tt>e1</tt> and <tt>e2</tt> such that <tt>e1.equals(e2)</tt>, and they typically allow multiple null elements if they allow null elements at all. It is not inconceivable that someone might wish to implement a list that prohibits duplicates, by throwing runtime exceptions when the user attempts to insert them, but we expect this usage to be rare.</p>
/// <p>The <tt>List</tt> interface places additional stipulations, beyond those specified in the <tt>Collection</tt> interface, on the contracts of the <tt>iterator</tt>, <tt>add</tt>, <tt>remove</tt>, <tt>equals</tt>, and <tt>hashCode</tt> methods. Declarations for other inherited methods are also included here for convenience.</p>
/// <p>The <tt>List</tt> interface provides four methods for positional (indexed) access to list elements. Lists (like Java arrays) are zero based. Note that these operations may execute in time proportional to the index value for some implementations (the <tt>LinkedList</tt> class, for example). Thus, iterating over the elements in a list is typically preferable to indexing through it if the caller does not know the implementation.</p>
/// <p>The <tt>List</tt> interface provides a special iterator, called a <tt>ListIterator</tt>, that allows element insertion and replacement, and bidirectional access in addition to the normal operations that the <tt>Iterator</tt> interface provides. A method is provided to obtain a list iterator that starts at a specified position in the list.</p>
/// <p>The <tt>List</tt> interface provides two methods to search for a specified object. From a performance standpoint, these methods should be used with caution. In many implementations they will perform costly linear searches.</p>
/// <p>The <tt>List</tt> interface provides two methods to efficiently insert and remove multiple elements at an arbitrary point in the list.</p>
/// <p>Note: While it is permissible for lists to contain themselves as elements, extreme caution is advised: the <tt>equals</tt> and <tt>hashCode</tt> methods are no longer well defined on such a list.</p>
/// <p>Some list implementations have restrictions on the elements that they may contain. For example, some implementations prohibit null elements, and some have restrictions on the types of their elements. Attempting to add an ineligible element throws an unchecked exception, typically <tt>NullPointerException</tt> or <tt>ClassCastException</tt>. Attempting to query the presence of an ineligible element may throw an exception, or it may simply return false; some implementations will exhibit the former behavior and some will exhibit the latter. More generally, attempting an operation on an ineligible element whose completion would not result in the insertion of an ineligible element into the list may throw an exception or it may succeed, at the option of the implementation. Such exceptions are marked as "optional" in the specification for this interface.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/List")?;
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
}

impl<'mc> JavaList<'mc> {
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_int(
        &self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn copy_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaList::from_raw(&jni, obj)
    }
    //

    pub fn index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn of_with_object(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
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
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/lang/Object;";
            let val_3 = jni::objects::JValueGen::Object(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Ljava/lang/Object;";
            let val_4 = jni::objects::JValueGen::Object(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "Ljava/lang/Object;";
            let val_5 = jni::objects::JValueGen::Object(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Ljava/lang/Object;";
            let val_6 = jni::objects::JValueGen::Object(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "Ljava/lang/Object;";
            let val_7 = jni::objects::JValueGen::Object(a);
            args.push(val_7);
        }
        if let Some(a) = arg7 {
            sig += "Ljava/lang/Object;";
            let val_8 = jni::objects::JValueGen::Object(a);
            args.push(val_8);
        }
        if let Some(a) = arg8 {
            sig += "Ljava/lang/Object;";
            let val_9 = jni::objects::JValueGen::Object(a);
            args.push(val_9);
        }
        if let Some(a) = arg9 {
            sig += "Ljava/lang/Object;";
            let val_10 = jni::objects::JValueGen::Object(a);
            args.push(val_10);
        }
        sig += ")Ljava/util/List;";
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "of", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaList::from_raw(&jni, obj)
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::util::JavaCollection<'mc>> for JavaList<'mc> {
    fn into(self) -> crate::util::JavaCollection<'mc> {
        crate::util::JavaCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaList into crate::util::JavaCollection")
    }
}
/// A class that represents an immutable universally unique identifier (UUID). A UUID represents a 128-bit value.
/// <p>There exist different variants of these global identifiers. The methods of this class are for manipulating the Leach-Salz variant, although the constructors allow the creation of any variant of UUID (described below).</p>
/// <p>The layout of a variant 2 (Leach-Salz) UUID is as follows: The most significant long consists of the following unsigned fields:</p>
/// <pre> 0xFFFFFFFF00000000 time_low
/// 0x00000000FFFF0000 time_mid
/// 0x000000000000F000 version
/// 0x0000000000000FFF time_hi
/// </pre> The least significant long consists of the following unsigned fields:
/// <pre> 0xC000000000000000 variant
/// 0x3FFF000000000000 clock_seq
/// 0x0000FFFFFFFFFFFF node
/// </pre>
/// <p>The variant field contains a value which identifies the layout of the <code>UUID</code>. The bit layout described above is valid only for a <code>UUID</code> with a variant value of 2, which indicates the Leach-Salz variant.</p>
/// <p>The version field holds a value that describes the type of this <code>UUID</code>. There are four different basic types of UUIDs: time-based, DCE security, name-based, and randomly generated UUIDs. These types have a version value of 1, 2, 3 and 4, respectively.</p>
/// <p>For more information including algorithms used to create <code>UUID</code>s, see <a href="http://www.ietf.org/rfc/rfc4122.txt"> <i>RFC&nbsp;4122: A Universally Unique IDentifier (UUID) URN Namespace</i></a>, section 4.2 "Algorithms for Creating a Time-Based UUID".</p>
pub struct JavaUUID<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaUUID<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaUUID<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaUUID from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/UUID")?;
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
}

impl<'mc> JavaUUID<'mc> {
    //

    pub fn name_uuidfrom_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<i8>,
    ) -> Result<crate::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(B)Ljava/util/UUID;");
        let cls = jni.find_class("java/util/UUID");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "nameUUIDFromBytes", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaUUID::from_raw(&jni, obj)
    }
    //

    pub fn least_significant_bits(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeastSignificantBits",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn most_significant_bits(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMostSignificantBits",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn clock_sequence(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clockSequence", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn variant(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "variant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn random_uuid(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let cls = jni.find_class("java/util/UUID");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "randomUUID", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaUUID::from_raw(&jni, obj)
    }
    //

    pub fn from_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/UUID;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/util/UUID");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "fromString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaUUID::from_raw(&jni, obj)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn version(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "version", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn compare_to_with_uuid(
        &self,
        arg0: impl Into<crate::util::JavaUUID<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn timestamp(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "timestamp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn node(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "node", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaUUID<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaUUID.toString: {}", err),
        }
    }
}

/// This class implements the <tt>Set</tt> interface, backed by a hash table (actually a <tt>HashMap</tt> instance). It makes no guarantees as to the iteration order of the set; in particular, it does not guarantee that the order will remain constant over time. This class permits the <tt>null</tt> element.
/// <p>This class offers constant time performance for the basic operations (<tt>add</tt>, <tt>remove</tt>, <tt>contains</tt> and <tt>size</tt>), assuming the hash function disperses the elements properly among the buckets. Iterating over this set requires time proportional to the sum of the <tt>HashSet</tt> instance's size (the number of elements) plus the "capacity" of the backing <tt>HashMap</tt> instance (the number of buckets). Thus, it's very important not to set the initial capacity too high (or the load factor too low) if iteration performance is important.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a hash set concurrently, and at least one of the threads modifies the set, it <i>must</i> be synchronized externally. This is typically accomplished by synchronizing on some object that naturally encapsulates the set. If no such object exists, the set should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedSet-java.util.Set-"><code>Collections.synchronizedSet</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the set:</p>
/// <pre> Set s = Collections.synchronizedSet(new HashSet(...));</pre>
/// <p>The iterators returned by this class's <tt>iterator</tt> method are <i>fail-fast</i>: if the set is modified at any time after the iterator is created, in any way except through the iterator's own <tt>remove</tt> method, the Iterator throws a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <tt>ConcurrentModificationException</tt> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaHashSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaHashSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaHashSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaHashSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/HashSet")?;
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
}

impl<'mc> JavaHashSet<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaHashSet<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaHashSet.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaSet<'mc>> for JavaHashSet<'mc> {
    fn into(self) -> crate::util::JavaSet<'mc> {
        crate::util::JavaSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaHashSet into crate::util::JavaSet")
    }
}
impl<'mc> Into<crate::util::JavaAbstractSet<'mc>> for JavaHashSet<'mc> {
    fn into(self) -> crate::util::JavaAbstractSet<'mc> {
        crate::util::JavaAbstractSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaHashSet into crate::util::JavaAbstractSet")
    }
}
/// This class provides a skeletal implementation of the <a title="interface in java.util" href="../../java/util/List.html"><code>List</code></a> interface to minimize the effort required to implement this interface backed by a "random access" data store (such as an array). For sequential access data (such as a linked list), <a title="class in java.util" href="../../java/util/AbstractSequentialList.html"><code>AbstractSequentialList</code></a> should be used in preference to this class.
/// <p>To implement an unmodifiable list, the programmer needs only to extend this class and provide implementations for the <a href="../../java/util/AbstractList.html#get-int-"><code>get(int)</code></a> and <a href="../../java/util/List.html#size--"><code>size()</code></a> methods.</p>
/// <p>To implement a modifiable list, the programmer must additionally override the <a href="../../java/util/AbstractList.html#set-int-E-"><code>set(int, E)</code></a> method (which otherwise throws an <code>UnsupportedOperationException</code>). If the list is variable-size the programmer must additionally override the <a href="../../java/util/AbstractList.html#add-int-E-"><code>add(int, E)</code></a> and <a href="../../java/util/AbstractList.html#remove-int-"><code>remove(int)</code></a> methods.</p>
/// <p>The programmer should generally provide a void (no argument) and collection constructor, as per the recommendation in the <a href="../../java/util/Collection.html" title="interface in java.util"><code>Collection</code></a> interface specification.</p>
/// <p>Unlike the other abstract collection implementations, the programmer does <i>not</i> have to provide an iterator implementation; the iterator and list iterator are implemented by this class, on top of the "random access" methods: <a href="../../java/util/AbstractList.html#get-int-"><code>get(int)</code></a>, <a href="../../java/util/AbstractList.html#set-int-E-"><code>set(int, E)</code></a>, <a href="../../java/util/AbstractList.html#add-int-E-"><code>add(int, E)</code></a> and <a href="../../java/util/AbstractList.html#remove-int-"><code>remove(int)</code></a>.</p>
/// <p>The documentation for each non-abstract method in this class describes its implementation in detail. Each of these methods may be overridden if the collection being implemented admits a more efficient implementation.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaAbstractList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaAbstractList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaAbstractList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaAbstractList from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/AbstractList")?;
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
}

impl<'mc> JavaAbstractList<'mc> {
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaAbstractList<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaAbstractList.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaList<'mc>> for JavaAbstractList<'mc> {
    fn into(self) -> crate::util::JavaList<'mc> {
        crate::util::JavaList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractList into crate::util::JavaList")
    }
}
impl<'mc> Into<crate::util::JavaAbstractCollection<'mc>> for JavaAbstractList<'mc> {
    fn into(self) -> crate::util::JavaAbstractCollection<'mc> {
        crate::util::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaAbstractList into crate::util::JavaAbstractCollection")
    }
}
/// A container object which may or may not contain a non-null value. If a value is present, <code>isPresent()</code> will return <code>true</code> and <code>get()</code> will return the value.
/// <p>Additional methods that depend on the presence or absence of a contained value are provided, such as <a href="../../java/util/Optional.html#orElse-T-"><code>orElse()</code></a> (return a default value if value not present) and <a href="../../java/util/Optional.html#ifPresent-java.util.function.Consumer-"><code>ifPresent()</code></a> (execute a block of code if the value is present).</p>
/// <p>This is a <a href="../lang/doc-files/ValueBased.html">value-based</a> class; use of identity-sensitive operations (including reference equality (<code>==</code>), identity hash code, or synchronization) on instances of <code>Optional</code> may have unpredictable results and should be avoided.</p>
pub struct JavaOptional<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaOptional<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaOptional<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaOptional from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Optional")?;
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
}

impl<'mc> JavaOptional<'mc> {
    //

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Supplier;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn or_else_get(
        &self,
        arg0: impl Into<crate::util::function::JavaSupplier<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Supplier;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElseGet",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn of_nullable(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let cls = jni.find_class("java/util/Optional");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "ofNullable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptional::from_raw(&jni, obj)
    }
    //

    pub fn if_present(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ifPresent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn map(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "map",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let cls = jni.find_class("java/util/Optional");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "of",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptional::from_raw(&jni, obj)
    }
    //

    pub fn filter(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "filter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn empty(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let cls = jni.find_class("java/util/Optional");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "empty", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaOptional::from_raw(&jni, obj)
    }
    //

    pub fn flat_map(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/Optional;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "flatMap",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_present(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPresent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn or_else(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "orElse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn or_else_throw_with_supplier(
        &self,
        arg0: std::option::Option<impl Into<crate::util::function::JavaSupplier<'mc>>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/function/Supplier;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "orElseThrow", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaOptional<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaOptional.toString: {}", err),
        }
    }
}

/// An iterator for lists that allows the programmer to traverse the list in either direction, modify the list during iteration, and obtain the iterator's current position in the list. A <code>ListIterator</code> has no current element; its <i>cursor position</i> always lies between the element that would be returned by a call to <code>previous()</code> and the element that would be returned by a call to <code>next()</code>. An iterator for a list of length <code>n</code> has <code>n+1</code> possible cursor positions, as illustrated by the carets (<code>^</code>) below:
/// <pre>Element(0) Element(1) Element(2) ... Element(n-1)
/// cursor positions:^^^^^
/// </pre> Note that the <a href="../../java/util/ListIterator.html#remove--"><code>remove()</code></a> and <a href="../../java/util/ListIterator.html#set-E-"><code>set(Object)</code></a> methods are <i>not</i> defined in terms of the cursor position; they are defined to operate on the last element returned by a call to <a href="../../java/util/ListIterator.html#next--"><code>next()</code></a> or <a href="../../java/util/ListIterator.html#previous--"><code>previous()</code></a>.
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaListIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaListIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaListIterator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaListIterator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/ListIterator")?;
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
}

impl<'mc> JavaListIterator<'mc> {
    //

    pub fn add(&self, arg0: jni::objects::JObject<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn set(&self, arg0: jni::objects::JObject<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextIndex", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn previous_index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "previousIndex", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn has_previous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPrevious", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn previous(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "previous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn for_each_remaining(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::util::JavaIterator<'mc>> for JavaListIterator<'mc> {
    fn into(self) -> crate::util::JavaIterator<'mc> {
        crate::util::JavaIterator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaListIterator into crate::util::JavaIterator")
    }
}
/// Doubly-linked list implementation of the <code>List</code> and <code>Deque</code> interfaces. Implements all optional list operations, and permits all elements (including <code>null</code>).
/// <p>All of the operations perform as could be expected for a doubly-linked list. Operations that index into the list will traverse the list from the beginning or the end, whichever is closer to the specified index.</p>
/// <p><strong>Note that this implementation is not synchronized.</strong> If multiple threads access a linked list concurrently, and at least one of the threads modifies the list structurally, it <i>must</i> be synchronized externally. (A structural modification is any operation that adds or deletes one or more elements; merely setting the value of an element is not a structural modification.) This is typically accomplished by synchronizing on some object that naturally encapsulates the list. If no such object exists, the list should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedList-java.util.List-"><code>Collections.synchronizedList</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access to the list:</p>
/// <pre> List list = Collections.synchronizedList(new LinkedList(...));</pre>
/// <p>The iterators returned by this class's <code>iterator</code> and <code>listIterator</code> methods are <i>fail-fast</i>: if the list is structurally modified at any time after the iterator is created, in any way except through the Iterator's own <code>remove</code> or <code>add</code> methods, the iterator will throw a <a href="../../java/util/ConcurrentModificationException.html" title="class in java.util"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaLinkedList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLinkedList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLinkedList<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLinkedList from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/LinkedList")?;
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
}

impl<'mc> JavaLinkedList<'mc> {
    //

    pub fn push(&self, arg0: jni::objects::JObject<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn pop(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn add_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn poll_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn offer_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_first_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_last_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn descending_iterator(
        &self,
    ) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Object;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "add", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get(&self, arg0: i32) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_index_of(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lastIndexOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<impl Into<crate::util::JavaCollection<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Collection;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addAll", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set(
        &self,
        arg0: i32,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "poll", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peek", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "element", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn list_iterator_with_int(
        &self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::JavaListIterator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/ListIterator;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "listIterator", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaListIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn sub_list(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subList",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn replace_all(
        &self,
        arg0: impl Into<crate::util::function::JavaUnaryOperator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/UnaryOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn sort(
        &self,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLinkedList<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLinkedList.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaList<'mc>> for JavaLinkedList<'mc> {
    fn into(self) -> crate::util::JavaList<'mc> {
        crate::util::JavaList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedList into crate::util::JavaList")
    }
}
impl<'mc> Into<crate::util::JavaDeque<'mc>> for JavaLinkedList<'mc> {
    fn into(self) -> crate::util::JavaDeque<'mc> {
        crate::util::JavaDeque::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedList into crate::util::JavaDeque")
    }
}
impl<'mc> Into<crate::util::JavaAbstractSequentialList<'mc>> for JavaLinkedList<'mc> {
    fn into(self) -> crate::util::JavaAbstractSequentialList<'mc> {
        crate::util::JavaAbstractSequentialList::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaLinkedList into crate::util::JavaAbstractSequentialList")
    }
}
/// This class expresses a <em>Language Range</em> defined in <a href="http://tools.ietf.org/html/rfc4647">RFC 4647 Matching of Language Tags</a>. A language range is an identifier which is used to select language tag(s) meeting specific requirements by using the mechanisms described in <a href="Locale.html#LocaleMatching">Locale Matching</a>. A list which represents a user's preferences and consists of language ranges is called a <em>Language Priority List</em>.
/// <p>There are two types of language ranges: basic and extended. In RFC 4647, the syntax of language ranges is expressed in <a href="http://tools.ietf.org/html/rfc4234">ABNF</a> as follows:</p>
/// <blockquote>
/// <pre> basic-language-range= (1*8ALPHA *("-" 1*8alphanum)) / "*"
/// extended-language-range = (1*8ALPHA / "*")
/// *("-" (1*8alphanum / "*"))
/// alphanum= ALPHA / DIGIT
/// </pre>
/// </blockquote> For example, <code>"en"</code> (English), <code>"ja-JP"</code> (Japanese, Japan), <code>"*"</code> (special language range which matches any language tag) are basic language ranges, whereas <code>"*-CH"</code> (any languages, Switzerland), <code>"es-*"</code> (Spanish, any regions), and <code>"zh-Hant-*"</code> (Traditional Chinese, any regions) are extended language ranges.
pub struct JavaLocaleLanguageRange<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLocaleLanguageRange<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLocaleLanguageRange<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLocaleLanguageRange from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Locale$LanguageRange")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLocaleLanguageRange object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLocaleLanguageRange<'mc> {
    //

    pub fn map_equivalents(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaList<'mc>>,
        arg1: impl Into<crate::util::JavaMap<'mc>>,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;Ljava/util/Map;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "mapEquivalents",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaList::from_raw(&jni, obj)
    }
    //

    pub fn range(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRange", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn weight(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn parse_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::util::JavaMap<'mc>>>,
    ) -> Result<crate::util::JavaList<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/Map;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Ljava/util/List;";
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "parse", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaList::from_raw(&jni, obj)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLocaleLanguageRange<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLocaleLanguageRange.toString: {}", err),
        }
    }
}

/// The root interface in the <i>collection hierarchy</i>. A collection represents a group of objects, known as its <i>elements</i>. Some collections allow duplicate elements and others do not. Some are ordered and others unordered. The JDK does not provide any <i>direct</i> implementations of this interface: it provides implementations of more specific subinterfaces like <tt>Set</tt> and <tt>List</tt>. This interface is typically used to pass collections around and manipulate them where maximum generality is desired.
/// <p><i>Bags</i> or <i>multisets</i> (unordered collections that may contain duplicate elements) should implement this interface directly.</p>
/// <p>All general-purpose <tt>Collection</tt> implementation classes (which typically implement <tt>Collection</tt> indirectly through one of its subinterfaces) should provide two "standard" constructors: a void (no arguments) constructor, which creates an empty collection, and a constructor with a single argument of type <tt>Collection</tt>, which creates a new collection with the same elements as its argument. In effect, the latter constructor allows the user to copy any collection, producing an equivalent collection of the desired implementation type. There is no way to enforce this convention (as interfaces cannot contain constructors) but all of the general-purpose <tt>Collection</tt> implementations in the Java platform libraries comply.</p>
/// <p>The "destructive" methods contained in this interface, that is, the methods that modify the collection on which they operate, are specified to throw <tt>UnsupportedOperationException</tt> if this collection does not support the operation. If this is the case, these methods may, but are not required to, throw an <tt>UnsupportedOperationException</tt> if the invocation would have no effect on the collection. For example, invoking the <a href="../../java/util/Collection.html#addAll-java.util.Collection-"><code>addAll(Collection)</code></a> method on an unmodifiable collection may, but is not required to, throw the exception if the collection to be added is empty.</p>
/// <p><a name="optional-restrictions"> Some collection implementations have restrictions on the elements that they may contain.</a> For example, some implementations prohibit null elements, and some have restrictions on the types of their elements. Attempting to add an ineligible element throws an unchecked exception, typically <tt>NullPointerException</tt> or <tt>ClassCastException</tt>. Attempting to query the presence of an ineligible element may throw an exception, or it may simply return false; some implementations will exhibit the former behavior and some will exhibit the latter. More generally, attempting an operation on an ineligible element whose completion would not result in the insertion of an ineligible element into the collection may throw an exception or it may succeed, at the option of the implementation. Such exceptions are marked as "optional" in the specification for this interface.</p>
/// <p>It is up to each collection to determine its own synchronization policy. In the absence of a stronger guarantee by the implementation, undefined behavior may result from the invocation of any method on a collection that is being mutated by another thread; this includes direct invocations, passing the collection to a method that might perform invocations, and using an existing iterator to examine the collection.</p>
/// <p>Many methods in Collections Framework interfaces are defined in terms of the <a href="../../java/lang/Object.html#equals-java.lang.Object-"><code>equals</code></a> method. For example, the specification for the <a href="../../java/util/Collection.html#contains-java.lang.Object-"><code>contains(Object o)</code></a> method says: "returns <tt>true</tt> if and only if this collection contains at least one element <tt>e</tt> such that <tt>(o==null ? e==null : o.equals(e))</tt>." This specification should <i>not</i> be construed to imply that invoking <tt>Collection.contains</tt> with a non-null argument <tt>o</tt> will cause <tt>o.equals(e)</tt> to be invoked for any element <tt>e</tt>. Implementations are free to implement optimizations whereby the <tt>equals</tt> invocation is avoided, for example, by first comparing the hash codes of the two elements. (The <a href="../../java/lang/Object.html#hashCode--"><code>Object.hashCode()</code></a> specification guarantees that two objects with unequal hash codes cannot be equal.) More generally, implementations of the various Collections Framework interfaces are free to take advantage of the specified behavior of underlying <a href="../../java/lang/Object.html" title="class in java.lang"><code>Object</code></a> methods wherever the implementor deems it appropriate.</p>
/// <p>Some collection operations which perform recursive traversal of the collection may fail with an exception for self-referential instances where the collection directly or indirectly contains itself. This includes the <code>clone()</code>, <code>equals()</code>, <code>hashCode()</code> and <code>toString()</code> methods. Implementations may optionally handle the self-referential scenario, however most current implementations do not do so.</p>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaCollection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaCollection from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Collection")?;
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
}

impl<'mc> JavaCollection<'mc> {
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// An Iterator specialized for <code>int</code> values.
///
/// This is a representation of an abstract class.
pub struct JavaPrimitiveIteratorOfInt<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfInt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaPrimitiveIteratorOfInt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfInt from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/PrimitiveIterator$OfInt")?;
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
}

impl<'mc> JavaPrimitiveIteratorOfInt<'mc> {
    //

    pub fn next_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn for_each_remaining_with_int_consumer(
        &self,
        arg0: impl Into<crate::util::function::JavaIntConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/IntConsumer;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "forEachRemaining", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// An Iterator specialized for <code>long</code> values.
///
/// This is a representation of an abstract class.
pub struct JavaPrimitiveIteratorOfLong<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaPrimitiveIteratorOfLong<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfLong from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/PrimitiveIterator$OfLong")?;
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
}

impl<'mc> JavaPrimitiveIteratorOfLong<'mc> {
    //

    pub fn next_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn for_each_remaining_with_long_consumer(
        &self,
        arg0: impl Into<crate::util::function::JavaLongConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/LongConsumer;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "forEachRemaining", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// An iterator over a collection. <code>Iterator</code> takes the place of <a href="../../java/util/Enumeration.html" title="interface in java.util"><code>Enumeration</code></a> in the Java Collections Framework. Iterators differ from enumerations in two ways:
/// <ul>
/// <li>Iterators allow the caller to remove elements from the underlying collection during the iteration with well-defined semantics.</li>
/// <li>Method names have been improved.</li>
/// </ul>
/// <p>This interface is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIterator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaIterator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Iterator")?;
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
}

impl<'mc> JavaIterator<'mc> {
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn for_each_remaining(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEachRemaining",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// A base type for primitive specializations of <code>Iterator</code>. Specialized subtypes are provided for <a href="../../java/util/PrimitiveIterator.OfInt.html" title="interface in java.util"><code>int</code></a>, <a title="interface in java.util" href="../../java/util/PrimitiveIterator.OfLong.html"><code>long</code></a>, and <a href="../../java/util/PrimitiveIterator.OfDouble.html" title="interface in java.util"><code>double</code></a> values.
/// <p>The specialized subtype default implementations of <a href="../../java/util/Iterator.html#next--"><code>Iterator.next()</code></a> and <a href="../../java/util/Iterator.html#forEachRemaining-java.util.function.Consumer-"><code>Iterator.forEachRemaining(java.util.function.Consumer)</code></a> box primitive values to instances of their corresponding wrapper class. Such boxing may offset any advantages gained when using the primitive specializations. To avoid boxing, the corresponding primitive-based methods should be used. For example, <a href="../../java/util/PrimitiveIterator.OfInt.html#nextInt--"><code>PrimitiveIterator.OfInt.nextInt()</code></a> and <a href="../../java/util/PrimitiveIterator.OfInt.html#forEachRemaining-java.util.function.IntConsumer-"><code>PrimitiveIterator.OfInt.forEachRemaining(java.util.function.IntConsumer)</code></a> should be used in preference to <a href="../../java/util/PrimitiveIterator.OfInt.html#next--"><code>PrimitiveIterator.OfInt.next()</code></a> and <a href="../../java/util/PrimitiveIterator.OfInt.html#forEachRemaining-java.util.function.Consumer-"><code>PrimitiveIterator.OfInt.forEachRemaining(java.util.function.Consumer)</code></a>.</p>
/// <p>Iteration of primitive values using boxing-based methods <a href="../../java/util/Iterator.html#next--"><code>next()</code></a> and <a href="../../java/util/Iterator.html#forEachRemaining-java.util.function.Consumer-"><code>forEachRemaining()</code></a>, does not affect the order in which the values, transformed to boxed values, are encountered.</p>
///
/// This is a representation of an abstract class.
pub struct JavaPrimitiveIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPrimitiveIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaPrimitiveIterator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIterator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/PrimitiveIterator")?;
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
}

impl<'mc> JavaPrimitiveIterator<'mc> {
    //

    pub fn for_each_remaining_with_consumer(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/Consumer;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "forEachRemaining", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
impl<'mc> Into<crate::util::JavaIterator<'mc>> for JavaPrimitiveIterator<'mc> {
    fn into(self) -> crate::util::JavaIterator<'mc> {
        crate::util::JavaIterator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaPrimitiveIterator into crate::util::JavaIterator")
    }
}
/// A map entry (key-value pair). The <tt>Map.entrySet</tt> method returns a collection-view of the map, whose elements are of this class. The <i>only</i> way to obtain a reference to a map entry is from the iterator of this collection-view. These <tt>Map.Entry</tt> objects are valid <i>only</i> for the duration of the iteration; more formally, the behavior of a map entry is undefined if the backing map has been modified after the entry was returned by the iterator, except through the <tt>setValue</tt> operation on the map entry.
///
/// This is a representation of an abstract class.
pub struct JavaMapEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaMapEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaMapEntry<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaMapEntry from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Map$Entry")?;
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
}

impl<'mc> JavaMapEntry<'mc> {
    //

    pub fn comparing_by_key_with_comparator(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::util::JavaComparator<'mc>>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Comparator;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/util/Comparator;";
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "comparingByKey", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn comparing_by_value_with_comparator(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::util::JavaComparator<'mc>>>,
    ) -> Result<crate::util::JavaComparator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Comparator;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/util/Comparator;";
        let cls = jni.find_class("java/util/Comparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "comparingByValue", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaComparator::from_raw(&jni, obj)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn copy_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaMapEntry<'mc>>,
    ) -> Result<crate::util::JavaMapEntry<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map$Entry;)Ljava/util/Map$Entry;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/Map$Entry");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "copyOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaMapEntry::from_raw(&jni, obj)
    }
    //

    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn key(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn set_value(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// A specialized <a title="interface in java.util" href="../../java/util/Set.html"><code>Set</code></a> implementation for use with enum types. All of the elements in an enum set must come from a single enum type that is specified, explicitly or implicitly, when the set is created. Enum sets are represented internally as bit vectors. This representation is extremely compact and efficient. The space and time performance of this class should be good enough to allow its use as a high-quality, typesafe alternative to traditional <tt>int</tt>-based "bit flags." Even bulk operations (such as <tt>containsAll</tt> and <tt>retainAll</tt>) should run very quickly if their argument is also an enum set.
/// <p>The iterator returned by the <tt>iterator</tt> method traverses the elements in their <i>natural order</i> (the order in which the enum constants are declared). The returned iterator is <i>weakly consistent</i>: it will never throw <a title="class in java.util" href="../../java/util/ConcurrentModificationException.html"><code>ConcurrentModificationException</code></a> and it may or may not show the effects of any modifications to the set that occur while the iteration is in progress.</p>
/// <p>Null elements are not permitted. Attempts to insert a null element will throw <a title="class in java.lang" href="../../java/lang/NullPointerException.html"><code>NullPointerException</code></a>. Attempts to test for the presence of a null element or to remove one will, however, function properly.</p>
/// <p>Like most collection implementations, <tt>EnumSet</tt> is not synchronized. If multiple threads access an enum set concurrently, and at least one of the threads modifies the set, it should be synchronized externally. This is typically accomplished by synchronizing on some object that naturally encapsulates the enum set. If no such object exists, the set should be "wrapped" using the <a href="../../java/util/Collections.html#synchronizedSet-java.util.Set-"><code>Collections.synchronizedSet(java.util.Set&lt;T&gt;)</code></a> method. This is best done at creation time, to prevent accidental unsynchronized access:</p>
/// <pre> Set&lt;MyEnum&gt; s = Collections.synchronizedSet(EnumSet.noneOf(MyEnum.class));
/// </pre>
/// <p>Implementation note: All basic operations execute in constant time. They are likely (though not guaranteed) to be much faster than their <a title="class in java.util" href="../../java/util/HashSet.html"><code>HashSet</code></a> counterparts. Even bulk operations execute in constant time if their argument is also an enum set.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaEnumSet<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaEnumSet<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaEnumSet<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaEnumSet from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/EnumSet")?;
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
}

impl<'mc> JavaEnumSet<'mc> {
    //

    pub fn clone(&self) -> Result<crate::util::JavaEnumSet<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/EnumSet;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaEnumSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "remove",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaEnumSet<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaEnumSet.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaAbstractSet<'mc>> for JavaEnumSet<'mc> {
    fn into(self) -> crate::util::JavaAbstractSet<'mc> {
        crate::util::JavaAbstractSet::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaEnumSet into crate::util::JavaAbstractSet")
    }
}
/// The class <code>Date</code> represents a specific instant in time, with millisecond precision.
/// <p>Prior to JDK&nbsp;1.1, the class <code>Date</code> had two additional functions. It allowed the interpretation of dates as year, month, day, hour, minute, and second values. It also allowed the formatting and parsing of date strings. Unfortunately, the API for these functions was not amenable to internationalization. As of JDK&nbsp;1.1, the <code>Calendar</code> class should be used to convert between dates and time fields and the <code>DateFormat</code> class should be used to format and parse date strings. The corresponding methods in <code>Date</code> are deprecated.</p>
/// <p>Although the <code>Date</code> class is intended to reflect coordinated universal time (UTC), it may not do so exactly, depending on the host environment of the Java Virtual Machine. Nearly all modern operating systems assume that 1&nbsp;day&nbsp;= 24&nbsp;×&nbsp;60&nbsp;×&nbsp;60&nbsp;= 86400 seconds in all cases. In UTC, however, about once every year or two there is an extra second, called a "leap second." The leap second is always added as the last second of the day, and always on December 31 or June 30. For example, the last minute of the year 1995 was 61 seconds long, thanks to an added leap second. Most computer clocks are not accurate enough to be able to reflect the leap-second distinction.</p>
/// <p>Some computer standards are defined in terms of Greenwich mean time (GMT), which is equivalent to universal time (UT). GMT is the "civil" name for the standard; UT is the "scientific" name for the same standard. The distinction between UTC and UT is that UTC is based on an atomic clock and UT is based on astronomical observations, which for all practical purposes is an invisibly fine hair to split. Because the earth's rotation is not uniform (it slows down and speeds up in complicated ways), UT does not always flow uniformly. Leap seconds are introduced as needed into UTC so as to keep UTC within 0.9 seconds of UT1, which is a version of UT with certain corrections applied. There are other time and date systems as well; for example, the time scale used by the satellite-based global positioning system (GPS) is synchronized to UTC but is <i>not</i> adjusted for leap seconds. An interesting source of further information is the U.S. Naval Observatory, particularly the Directorate of Time at:</p>
/// <blockquote>
/// <pre> <a href="http://tycho.usno.navy.mil">http://tycho.usno.navy.mil</a>
/// </pre>
/// </blockquote>
/// <p>and their definitions of "Systems of Time" at:</p>
/// <blockquote>
/// <pre> <a href="http://tycho.usno.navy.mil/systime.html">http://tycho.usno.navy.mil/systime.html</a>
/// </pre>
/// </blockquote>
/// <p>In all methods of class <code>Date</code> that accept or return year, month, date, hours, minutes, and seconds values, the following representations are used:</p>
/// <ul>
/// <li>A year <i>y</i> is represented by the integer <i>y</i>&nbsp;<code>-&nbsp;1900</code>.</li>
/// <li>A month is represented by an integer from 0 to 11; 0 is January, 1 is February, and so forth; thus 11 is December.</li>
/// <li>A date (day of month) is represented by an integer from 1 to 31 in the usual manner.</li>
/// <li>An hour is represented by an integer from 0 to 23. Thus, the hour from midnight to 1 a.m. is hour 0, and the hour from noon to 1 p.m. is hour 12.</li>
/// <li>A minute is represented by an integer from 0 to 59 in the usual manner.</li>
/// <li>A second is represented by an integer from 0 to 61; the values 60 and 61 occur only for leap seconds and even then only in Java implementations that actually track leap seconds correctly. Because of the manner in which leap seconds are currently introduced, it is extremely unlikely that two leap seconds will occur in the same minute, but this specification follows the date and time conventions for ISO C.</li>
/// </ul>
/// <p>In all cases, arguments given to methods for these purposes need not fall within the indicated ranges; for example, a date may be specified as January 32 and is interpreted as meaning February 1.</p>
pub struct JavaDate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Date")?;
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
}

impl<'mc> JavaDate<'mc> {
    //

    pub fn date(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_time(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_date(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn month(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMonth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_month(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMonth",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn hours(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHours", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_hours(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHours",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn minutes(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinutes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_minutes(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinutes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_seconds(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeconds",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_year(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYear",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn day(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn to_locale_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toLocaleString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn to_gmtstring(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toGMTString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn timezone_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTimezoneOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn time(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn year(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYear", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn seconds(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSeconds", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn to_instant(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/time/Instant;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toInstant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn utc(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(IIIIII)J");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.into());
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "UTC",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn parse(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)J");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn before(
        &self,
        arg0: impl Into<crate::util::JavaDate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Date;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "before",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn after(
        &self,
        arg0: impl Into<crate::util::JavaDate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Date;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "after",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn from(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::JavaDate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/time/Instant;)Ljava/util/Date;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let cls = jni.find_class("java/util/Date");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "from",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::JavaDate::from_raw(&jni, obj)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaDate<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaDate.toString: {}", err),
        }
    }
}

/// Resizable-array implementation of the <a title="interface in java.util" href="../../java/util/Deque.html"><code>Deque</code></a> interface. Array deques have no capacity restrictions; they grow as necessary to support usage. They are not thread-safe; in the absence of external synchronization, they do not support concurrent access by multiple threads. Null elements are prohibited. This class is likely to be faster than <a href="../../java/util/Stack.html" title="class in java.util"><code>Stack</code></a> when used as a stack, and faster than <a title="class in java.util" href="../../java/util/LinkedList.html"><code>LinkedList</code></a> when used as a queue.
/// <p>Most <code>ArrayDeque</code> operations run in amortized constant time. Exceptions include <a href="../../java/util/ArrayDeque.html#remove-java.lang.Object-"><code>remove</code></a>, <a href="../../java/util/ArrayDeque.html#removeFirstOccurrence-java.lang.Object-"><code>removeFirstOccurrence</code></a>, <a href="../../java/util/ArrayDeque.html#removeLastOccurrence-java.lang.Object-"><code>removeLastOccurrence</code></a>, <a href="../../java/util/ArrayDeque.html#contains-java.lang.Object-"><code>contains</code></a>, <a href="../../java/util/ArrayDeque.html#iterator--"><code>iterator.remove()</code></a>, and the bulk operations, all of which run in linear time.</p>
/// <p>The iterators returned by this class's <code>iterator</code> method are <i>fail-fast</i>: If the deque is modified at any time after the iterator is created, in any way except through the iterator's own <code>remove</code> method, the iterator will generally throw a <a title="class in java.util" href="../../java/util/ConcurrentModificationException.html"><code>ConcurrentModificationException</code></a>. Thus, in the face of concurrent modification, the iterator fails quickly and cleanly, rather than risking arbitrary, non-deterministic behavior at an undetermined time in the future.</p>
/// <p>Note that the fail-fast behavior of an iterator cannot be guaranteed as it is, generally speaking, impossible to make any hard guarantees in the presence of unsynchronized concurrent modification. Fail-fast iterators throw <code>ConcurrentModificationException</code> on a best-effort basis. Therefore, it would be wrong to write a program that depended on this exception for its correctness: <i>the fail-fast behavior of iterators should be used only to detect bugs.</i></p>
/// <p>This class and its iterator implement all of the <em>optional</em> methods of the <a href="../../java/util/Collection.html" title="interface in java.util"><code>Collection</code></a> and <a href="../../java/util/Iterator.html" title="interface in java.util"><code>Iterator</code></a> interfaces.</p>
/// <p>This class is a member of the <a href="../../../technotes/guides/collections/index.html"> Java Collections Framework</a>.</p>
pub struct JavaArrayDeque<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaArrayDeque<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaArrayDeque<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaArrayDeque from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/ArrayDeque")?;
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
}

impl<'mc> JavaArrayDeque<'mc> {
    //

    pub fn push(&self, arg0: jni::objects::JObject<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "push",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn pop(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pop", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn add_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn add_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn poll_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn poll_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "pollLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn offer_last(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerLast",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_first(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekFirst", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_first_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeFirstOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer_first(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offerFirst",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek_last(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peekLast", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_last_occurrence(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeLastOccurrence",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn offer(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn descending_iterator(
        &self,
    ) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "descendingIterator",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_with_object(
        &self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/Object;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn clone(&self) -> Result<crate::util::JavaArrayDeque<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/ArrayDeque;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaArrayDeque::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    //

    pub fn iterator(&self) -> Result<crate::util::JavaIterator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Iterator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "iterator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaIterator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn contains(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn for_each(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "forEach",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn poll(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "poll", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn peek(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "peek", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_if(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeIf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn element(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "element", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn retain_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retainAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn contains_all(
        &self,
        arg0: impl Into<crate::util::JavaCollection<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Collection;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "containsAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaArrayDeque<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaArrayDeque.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::JavaDeque<'mc>> for JavaArrayDeque<'mc> {
    fn into(self) -> crate::util::JavaDeque<'mc> {
        crate::util::JavaDeque::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaArrayDeque into crate::util::JavaDeque")
    }
}
impl<'mc> Into<crate::util::JavaAbstractCollection<'mc>> for JavaArrayDeque<'mc> {
    fn into(self) -> crate::util::JavaAbstractCollection<'mc> {
        crate::util::JavaAbstractCollection::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaArrayDeque into crate::util::JavaAbstractCollection")
    }
}
/// An Iterator specialized for <code>double</code> values.
///
/// This is a representation of an abstract class.
pub struct JavaPrimitiveIteratorOfDouble<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPrimitiveIteratorOfDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaPrimitiveIteratorOfDouble<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaPrimitiveIteratorOfDouble from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/PrimitiveIterator$OfDouble")?;
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
}

impl<'mc> JavaPrimitiveIteratorOfDouble<'mc> {
    //

    pub fn next_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn for_each_remaining_with_double_consumer(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleConsumer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/function/DoubleConsumer;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "forEachRemaining", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "next", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_next(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// <code>Builder</code> is used to build instances of <code>Locale</code> from values configured by the setters. Unlike the <code>Locale</code> constructors, the <code>Builder</code> checks if a value configured by a setter satisfies the syntax requirements defined by the <code>Locale</code> class. A <code>Locale</code> object created by a <code>Builder</code> is well-formed and can be transformed to a well-formed IETF BCP 47 language tag without losing information.
/// <p><b>Note:</b> The <code>Locale</code> class does not provide any syntactic restrictions on variant, while BCP 47 requires each variant subtag to be 5 to 8 alphanumerics or a single numeric followed by 3 alphanumerics. The method <code>setVariant</code> throws <code>IllformedLocaleException</code> for a variant that does not satisfy this restriction. If it is necessary to support such a variant, use a Locale constructor. However, keep in mind that a <code>Locale</code> object created this way might lose the variant information when transformed to a BCP 47 language tag.</p>
/// <p>The following example shows how to create a <code>Locale</code> object with the <code>Builder</code>.</p>
/// <blockquote>
/// <pre> Locale aLocale = new Builder().setLanguage("sr").setScript("Latn").setRegion("RS").build();
/// </pre>
/// </blockquote>
/// <p>Builders can be reused; <code>clear()</code> resets all fields to their default values.</p>
pub struct JavaLocaleBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLocaleBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLocaleBuilder<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLocaleBuilder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/Locale$Builder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLocaleBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLocaleBuilder<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("java/util/Locale$Builder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&jni, res)
    }
    //

    pub fn set_language(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLanguage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_script(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScript",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_extension(
        &self,
        arg0: u16,
        arg1: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(CLjava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtension",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_unicode_locale_keyword(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnicodeLocaleKeyword",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_unicode_locale_attribute(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addUnicodeLocaleAttribute",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_unicode_locale_attribute(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeUnicodeLocaleAttribute",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear_extensions(
        &self,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Locale$Builder;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearExtensions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_language_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLanguageTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_locale(
        &self,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Locale;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn build(&self) -> Result<crate::util::JavaLocale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Locale;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "build", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocale::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_variant(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVariant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_region(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/Locale$Builder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRegion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn clear(&self) -> Result<crate::util::JavaLocaleBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Locale$Builder;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaLocaleBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for JavaLocaleBuilder<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaLocaleBuilder.toString: {}", err),
        }
    }
}

pub mod function;
pub mod logging;
pub mod random;
pub mod regex;
