pub mod bukkit;
pub mod bungee;
pub mod macros;
use jni::{
    descriptors::Desc,
    errors::Error,
    objects::*,
    signature::{JavaType, ReturnType},
    strings::{JNIString, JavaStr},
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort, jsize, jvalue},
    JNIEnv, JNIVersion, MonitorGuard, NativeMethod,
};
use std::cell::RefCell;

/// Wrapper struct for [JNIEnv](jni::JNIEnv) that has interior mutability and several other helper functions.
pub struct SharedJNIEnv<'mc> {
    jni: RefCell<jni::JNIEnv<'mc>>,
}

impl<'mc> SharedJNIEnv<'mc> {
    pub fn new(jni: jni::JNIEnv<'mc>) -> Self {
        Self {
            jni: RefCell::new(jni),
        }
    }
    pub fn define_class<S>(
        &self,
        name: S,
        loader: &JObject<'_>,
        buf: &[u8],
    ) -> Result<JClass<'mc>, jni::errors::Error>
    where
        S: Into<JNIString>,
    {
        self.jni.borrow_mut().define_class(name, loader, buf)
    }
    pub fn define_unnamed_class(
        &self,
        loader: &JObject<'_>,
        buf: &[u8],
    ) -> Result<JClass<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().define_unnamed_class(loader, buf)
    }
    pub fn define_class_bytearray<S>(
        &self,
        name: S,
        loader: &JObject<'_>,
        buf: &AutoElements<'_, '_, '_, jbyte>,
    ) -> Result<JClass<'mc>, jni::errors::Error>
    where
        S: Into<JNIString>,
    {
        self.jni
            .borrow_mut()
            .define_class_bytearray(name, loader, buf)
    }
    pub fn find_class<S>(&self, name: S) -> Result<JClass<'mc>, jni::errors::Error>
    where
        S: Into<JNIString>,
    {
        self.jni.borrow_mut().find_class(name)
    }
    pub fn get_superclass<'other_local, T>(
        &self,
        class: T,
    ) -> Result<Option<JClass<'mc>>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni.borrow_mut().get_superclass(class)
    }
    pub fn is_assignable_from<'other_local_1, 'other_local_2, T, U>(
        &self,
        class1: T,
        class2: U,
    ) -> Result<bool, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local_1>>,
        U: Desc<'mc, JClass<'other_local_2>>,
    {
        self.jni.borrow_mut().is_assignable_from(class1, class2)
    }
    pub fn is_instance_of<'other_local_1, 'other_local_2, O, T>(
        &self,
        object: O,
        class: T,
    ) -> Result<bool, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local_1>>,
        T: Desc<'mc, JClass<'other_local_2>>,
    {
        self.jni.borrow_mut().is_instance_of(object, class)
    }

    pub fn throw<'other_local, E>(&self, obj: E) -> Result<(), jni::errors::Error>
    where
        E: Desc<'mc, JThrowable<'other_local>>,
    {
        self.jni.borrow_mut().throw(obj)
    }
    pub fn throw_new<'other_local, S, T>(&self, class: T, msg: S) -> Result<(), jni::errors::Error>
    where
        S: Into<JNIString>,
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni.borrow_mut().throw_new(class, msg)
    }
    pub fn exception_occurred(&self) -> Result<JThrowable<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().exception_occurred()
    }
    pub unsafe fn new_direct_byte_buffer(
        &self,
        data: *mut u8,
        len: usize,
    ) -> Result<JByteBuffer<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_direct_byte_buffer(data, len)
    }
    pub fn with_local_frame<F, T, E>(&self, capacity: i32, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut JNIEnv<'_>) -> Result<T, E>,
        E: From<Error>,
    {
        self.jni.borrow_mut().with_local_frame(capacity, f)
    }
    pub fn with_local_frame_returning_local<F, E>(
        &self,
        capacity: i32,
        f: F,
    ) -> Result<JObject<'mc>, E>
    where
        F: for<'new_local> FnOnce(&mut JNIEnv<'new_local>) -> Result<JObject<'new_local>, E>,
        E: From<Error>,
    {
        self.jni
            .borrow_mut()
            .with_local_frame_returning_local(capacity, f)
    }
    pub fn alloc_object<'other_local, T>(
        &self,
        class: T,
    ) -> Result<JObject<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni.borrow_mut().alloc_object(class)
    }
    pub fn get_method_id<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JMethodID, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.jni.borrow_mut().get_method_id(class, name, sig)
    }
    pub fn get_static_method_id<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JStaticMethodID, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.jni.borrow_mut().get_static_method_id(class, name, sig)
    }
    pub fn get_field_id<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JFieldID, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.jni.borrow_mut().get_field_id(class, name, sig)
    }
    pub fn get_static_field_id<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
    ) -> Result<JStaticFieldID, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString>,
    {
        self.jni.borrow_mut().get_static_field_id(class, name, sig)
    }
    pub unsafe fn call_static_method_unchecked<'other_local, T, U>(
        &self,
        class: T,
        method_id: U,
        ret: ReturnType,
        args: &[jvalue],
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Desc<'mc, JStaticMethodID>,
    {
        self.jni
            .borrow_mut()
            .call_static_method_unchecked(class, method_id, ret, args)
    }
    pub unsafe fn call_method_unchecked<'other_local, O, T>(
        &self,
        obj: O,
        method_id: T,
        ret: ReturnType,
        args: &[jvalue],
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'mc, JMethodID>,
    {
        self.jni
            .borrow_mut()
            .call_method_unchecked(obj, method_id, ret, args)
    }
    pub fn call_method<'other_local, O, S, T>(
        &self,
        obj: O,
        name: S,
        sig: T,
        args: &[JValue<'_, '_>],
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        self.jni.borrow_mut().call_method(obj, name, sig, args)
    }
    pub fn call_static_method<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
        args: &[JValue<'_, '_>],
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        self.jni
            .borrow_mut()
            .call_static_method(class, name, sig, args)
    }
    pub fn new_object<'other_local, T, U>(
        &self,
        class: T,
        ctor_sig: U,
        ctor_args: &[JValue<'_, '_>],
    ) -> Result<JObject<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString> + AsRef<str>,
    {
        self.jni.borrow_mut().new_object(class, ctor_sig, ctor_args)
    }
    pub unsafe fn new_object_unchecked<'other_local, T>(
        &self,
        class: T,
        ctor_id: JMethodID,
        ctor_args: &[jvalue],
    ) -> Result<JObject<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni
            .borrow_mut()
            .new_object_unchecked(class, ctor_id, ctor_args)
    }
    pub fn get_list<'other_local_1, 'obj_ref>(
        &self,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JList<'mc, 'other_local_1, 'obj_ref>, jni::errors::Error>
    where
        'other_local_1: 'obj_ref,
    {
        self.jni.borrow_mut().get_list(obj)
    }
    pub fn get_map<'other_local_1, 'obj_ref>(
        &self,
        obj: &'obj_ref JObject<'other_local_1>,
    ) -> Result<JMap<'mc, 'other_local_1, 'obj_ref>, jni::errors::Error>
    where
        'other_local_1: 'obj_ref,
    {
        self.jni.borrow_mut().get_map(obj)
    }
    pub fn get_string<'other_local: 'obj_ref, 'obj_ref>(
        &self,
        obj: &'obj_ref JString<'other_local>,
    ) -> Result<JavaStr<'mc, 'other_local, 'obj_ref>, jni::errors::Error> {
        self.jni.borrow_mut().get_string(obj)
    }
    pub fn new_object_array<'other_local_1, 'other_local_2, T, U>(
        &self,
        length: jsize,
        element_class: T,
        initial_element: U,
    ) -> Result<JObjectArray<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local_2>>,
        U: AsRef<JObject<'other_local_1>>,
    {
        self.jni
            .borrow_mut()
            .new_object_array(length, element_class, initial_element)
    }
    pub fn get_object_array_element<'other_local>(
        &self,
        array: impl AsRef<JObjectArray<'other_local>>,
        index: jsize,
    ) -> Result<JObject<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().get_object_array_element(array, index)
    }
    pub fn get_field_unchecked<'other_local, O, T>(
        &self,
        obj: O,
        field: T,
        ty: ReturnType,
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'mc, JFieldID>,
    {
        self.jni.borrow_mut().get_field_unchecked(obj, field, ty)
    }
    pub fn set_field_unchecked<'other_local, O, T>(
        &self,
        obj: O,
        field: T,
        val: JValue<'_, '_>,
    ) -> Result<(), jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        T: Desc<'mc, JFieldID>,
    {
        self.jni.borrow_mut().set_field_unchecked(obj, field, val)
    }
    pub fn get_field<'other_local, O, S, T>(
        &self,
        obj: O,
        name: S,
        ty: T,
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        self.jni.borrow_mut().get_field(obj, name, ty)
    }
    pub fn set_field<'other_local, O, S, T>(
        &self,
        obj: O,
        name: S,
        ty: T,
        val: JValue<'_, '_>,
    ) -> Result<(), jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Into<JNIString> + AsRef<str>,
    {
        self.jni.borrow_mut().set_field(obj, name, ty, val)
    }
    pub fn get_static_field_unchecked<'other_local, T, U>(
        &self,
        class: T,
        field: U,
        ty: JavaType,
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Desc<'mc, JStaticFieldID>,
    {
        self.jni
            .borrow_mut()
            .get_static_field_unchecked(class, field, ty)
    }
    pub fn get_static_field<'other_local, T, U, V>(
        &self,
        class: T,
        field: U,
        sig: V,
    ) -> Result<JValueOwned<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Into<JNIString>,
        V: Into<JNIString> + AsRef<str>,
    {
        self.jni.borrow_mut().get_static_field(class, field, sig)
    }
    pub fn set_static_field<'other_local, T, U>(
        &self,
        class: T,
        field: U,
        value: JValue<'_, '_>,
    ) -> Result<(), jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
        U: Desc<'mc, JStaticFieldID>,
    {
        self.jni.borrow_mut().set_static_field(class, field, value)
    }
    pub unsafe fn set_rust_field<'other_local, O, S, T>(
        &self,
        obj: O,
        field: S,
        rust_object: T,
    ) -> Result<(), jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: AsRef<str>,
        T: Send + 'static,
    {
        self.jni
            .borrow_mut()
            .set_rust_field(obj, field, rust_object)
    }
    /*pub unsafe fn get_rust_field<'other_local, O, S, T>(
        &self,
        obj: O,
        field: S,
    ) -> Result<MutexGuard<'_, T>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: Into<JNIString>,
        T: Send + 'static,
    {
        self.jni.borrow_mut().get_rust_field(obj, field)
    }*/
    pub unsafe fn take_rust_field<'other_local, O, S, T>(
        &self,
        obj: O,
        field: S,
    ) -> Result<T, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
        S: AsRef<str>,
        T: Send + 'static,
    {
        self.jni.borrow_mut().take_rust_field(obj, field)
    }
    pub fn register_native_methods<'other_local, T>(
        &self,
        class: T,
        methods: &[NativeMethod],
    ) -> Result<(), jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni
            .borrow_mut()
            .register_native_methods(class, methods)
    }
    pub fn unregister_native_methods<'other_local, T>(
        &self,
        class: T,
    ) -> Result<(), jni::errors::Error>
    where
        T: Desc<'mc, JClass<'other_local>>,
    {
        self.jni.borrow_mut().unregister_native_methods(class)
    }
    pub unsafe fn get_array_elements<'other_local, 'array, T: TypeArray>(
        &self,
        array: &'array JPrimitiveArray<'other_local, T>,
        mode: ReleaseMode,
    ) -> Result<AutoElements<'mc, 'other_local, 'array, T>, jni::errors::Error> {
        self.jni.borrow_mut().get_array_elements(array, mode)
    }
    /*pub unsafe fn get_array_elements_critical<'other_local, 'array, 'env, T: TypeArray>(
        &'env mut self,
        array: &'array JPrimitiveArray<'other_local, T>,
        mode: ReleaseMode,
    ) -> Result<AutoElementsCritical<'mc, 'other_local, 'array, 'env, T>, jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_array_elements_critical(array, mode)
    }*/

    pub fn get_raw(&self) -> *mut jni::sys::JNIEnv {
        self.jni.borrow_mut().get_raw()
    }

    pub fn get_version(&self) -> Result<JNIVersion, jni::errors::Error> {
        self.jni.borrow_mut().get_version()
    }
    pub fn is_same_object<'other_local_1, 'other_local_2, O, T>(
        &self,
        ref1: O,
        ref2: T,
    ) -> Result<bool, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local_1>>,
        T: AsRef<JObject<'other_local_2>>,
    {
        self.jni.borrow_mut().is_same_object(ref1, ref2)
    }
    pub fn exception_describe(&self) -> Result<(), jni::errors::Error> {
        self.jni.borrow_mut().exception_describe()
    }
    pub fn exception_clear(&self) -> Result<(), jni::errors::Error> {
        self.jni.borrow_mut().exception_clear()
    }
    pub fn fatal_error<S: Into<JNIString>>(&self, msg: S) -> ! {
        self.jni.borrow_mut().fatal_error(msg)
    }
    pub fn exception_check(&self) -> Result<bool, jni::errors::Error> {
        self.jni.borrow_mut().exception_check()
    }

    pub fn get_direct_buffer_capacity(
        &self,
        buf: &JByteBuffer<'_>,
    ) -> Result<usize, jni::errors::Error> {
        self.jni.borrow_mut().get_direct_buffer_capacity(buf)
    }

    pub fn new_global_ref<'other_local, O>(&self, obj: O) -> Result<GlobalRef, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
    {
        self.jni.borrow_mut().new_global_ref(obj)
    }

    pub fn new_weak_ref<'other_local, O>(
        &self,
        obj: O,
    ) -> Result<Option<WeakRef>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
    {
        self.jni.borrow_mut().new_weak_ref(obj)
    }

    pub fn new_local_ref<'other_local, O>(&self, obj: O) -> Result<JObject<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
    {
        self.jni.borrow_mut().new_local_ref(obj)
    }

    pub fn auto_local<O>(&self, obj: O) -> AutoLocal<'mc, O>
    where
        O: Into<JObject<'mc>>,
    {
        self.jni.borrow_mut().auto_local(obj)
    }

    pub fn push_local_frame(&self, capacity: i32) -> Result<(), jni::errors::Error> {
        self.jni.borrow_mut().push_local_frame(capacity)
    }

    pub fn get_object_class<'other_local, O>(
        &self,
        obj: O,
    ) -> Result<jni::objects::JClass<'_>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
    {
        self.jni.borrow_mut().get_object_class(obj)
    }

    pub fn new_string<S: Into<JNIString>>(
        &self,
        from: S,
    ) -> Result<JString<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_string(from)
    }

    pub fn set_object_array_element<'other_local_1, 'other_local_2>(
        &self,
        array: impl AsRef<JObjectArray<'other_local_1>>,
        index: jsize,
        value: impl AsRef<JObject<'other_local_2>>,
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_object_array_element(array, index, value)
    }
    pub fn byte_array_from_slice(&self, buf: &[u8]) -> Result<JByteArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().byte_array_from_slice(buf)
    }

    pub fn convert_byte_array<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
    ) -> Result<Vec<u8>, jni::errors::Error> {
        self.jni.borrow_mut().convert_byte_array(array)
    }
    pub fn new_boolean_array(
        &self,
        length: jsize,
    ) -> Result<JBooleanArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_boolean_array(length)
    }

    pub fn new_byte_array(&self, length: jsize) -> Result<JByteArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_byte_array(length)
    }

    pub fn new_char_array(&self, length: jsize) -> Result<JCharArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_char_array(length)
    }

    pub fn new_short_array(&self, length: jsize) -> Result<JShortArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_short_array(length)
    }

    pub fn new_int_array(&self, length: jsize) -> Result<JIntArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_int_array(length)
    }

    pub fn new_long_array(&self, length: jsize) -> Result<JLongArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_long_array(length)
    }

    pub fn new_float_array(&self, length: jsize) -> Result<JFloatArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_float_array(length)
    }

    pub fn new_double_array(&self, length: jsize) -> Result<JDoubleArray<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().new_double_array(length)
    }

    pub fn get_boolean_array_region<'other_local>(
        &self,
        array: impl AsRef<JBooleanArray<'other_local>>,
        start: jsize,
        buf: &mut [jboolean],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_boolean_array_region(array, start, buf)
    }
    pub fn get_byte_array_region<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
        start: jsize,
        buf: &mut [jbyte],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_byte_array_region(array, start, buf)
    }
    pub fn get_char_array_region<'other_local>(
        &self,
        array: impl AsRef<JCharArray<'other_local>>,
        start: jsize,
        buf: &mut [jchar],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_char_array_region(array, start, buf)
    }
    pub fn get_short_array_region<'other_local>(
        &self,
        array: impl AsRef<JShortArray<'other_local>>,
        start: jsize,
        buf: &mut [jshort],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_short_array_region(array, start, buf)
    }
    pub fn get_int_array_region<'other_local>(
        &self,
        array: impl AsRef<JIntArray<'other_local>>,
        start: jsize,
        buf: &mut [jint],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_int_array_region(array, start, buf)
    }
    pub fn get_long_array_region<'other_local>(
        &self,
        array: impl AsRef<JLongArray<'other_local>>,
        start: jsize,
        buf: &mut [jlong],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_long_array_region(array, start, buf)
    }
    pub fn get_float_array_region<'other_local>(
        &self,
        array: impl AsRef<JFloatArray<'other_local>>,
        start: jsize,
        buf: &mut [jfloat],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_float_array_region(array, start, buf)
    }
    pub fn get_double_array_region<'other_local>(
        &self,
        array: impl AsRef<JDoubleArray<'other_local>>,
        start: jsize,
        buf: &mut [jdouble],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .get_double_array_region(array, start, buf)
    }
    pub fn set_boolean_array_region<'other_local>(
        &self,
        array: impl AsRef<JBooleanArray<'other_local>>,
        start: jsize,
        buf: &[jboolean],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_boolean_array_region(array, start, buf)
    }

    pub fn set_byte_array_region<'other_local>(
        &self,
        array: impl AsRef<JByteArray<'other_local>>,
        start: jsize,
        buf: &[jbyte],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_byte_array_region(array, start, buf)
    }

    pub fn set_char_array_region<'other_local>(
        &self,
        array: impl AsRef<JCharArray<'other_local>>,
        start: jsize,
        buf: &[jchar],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_char_array_region(array, start, buf)
    }

    pub fn set_short_array_region<'other_local>(
        &self,
        array: impl AsRef<JShortArray<'other_local>>,
        start: jsize,
        buf: &[jshort],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_short_array_region(array, start, buf)
    }

    pub fn set_int_array_region<'other_local>(
        &self,
        array: impl AsRef<JIntArray<'other_local>>,
        start: jsize,
        buf: &[jint],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_int_array_region(array, start, buf)
    }

    pub fn set_long_array_region<'other_local>(
        &self,
        array: impl AsRef<JLongArray<'other_local>>,
        start: jsize,
        buf: &[jlong],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_long_array_region(array, start, buf)
    }

    pub fn set_float_array_region<'other_local>(
        &self,
        array: impl AsRef<JFloatArray<'other_local>>,
        start: jsize,
        buf: &[jfloat],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_float_array_region(array, start, buf)
    }

    pub fn set_double_array_region<'other_local>(
        &self,
        array: impl AsRef<JDoubleArray<'other_local>>,
        start: jsize,
        buf: &[jdouble],
    ) -> Result<(), jni::errors::Error> {
        self.jni
            .borrow_mut()
            .set_double_array_region(array, start, buf)
    }

    pub fn lock_obj<'other_local, O>(&self, obj: O) -> Result<MonitorGuard<'mc>, jni::errors::Error>
    where
        O: AsRef<JObject<'other_local>>,
    {
        self.jni.borrow_mut().lock_obj(obj)
    }

    pub fn get_native_interface(&self) -> *mut jni::sys::JNIEnv {
        self.jni.borrow_mut().get_native_interface()
    }

    pub fn get_java_vm(&self) -> Result<jni::JavaVM, jni::errors::Error> {
        self.jni.borrow_mut().get_java_vm()
    }

    pub fn ensure_local_capacity(&self, capacity: jint) -> Result<(), jni::errors::Error> {
        self.jni.borrow_mut().ensure_local_capacity(capacity)
    }

    pub fn translate_error(
        &self,
        res: Result<jni::objects::JValueGen<jni::objects::JObject<'mc>>, jni::errors::Error>,
    ) -> Result<jni::objects::JValueGen<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>
    {
        let mut jni = self.jni.borrow_mut();
        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                let exp = jni.exception_occurred()?;
                if !exp.is_null() {
                    jni.exception_clear()?;
                    let _message = String::new();
                    let obj = jni.call_method(&exp, "getMessage", "()Ljava/lang/String;", &[])?;
                    let mut message = format!(
                        "{}",
                        jni.get_string(unsafe {
                            &jni::objects::JString::from_raw(obj.as_jni().l)
                        })?
                        .to_string_lossy()
                        .to_string()
                    );
                    // Is there a cause?
                    let cause = jni
                        .call_method(&exp, "getCause", "()Ljava/lang/Throwable;", &[])?
                        .l()
                        .unwrap();
                    if !&cause.is_null() {
                        message += "\ncaused by: ";
                        let cause_obj =
                            jni.call_method(&cause, "getMessage", "()Ljava/lang/String;", &[])?;
                        message += format!(
                            "{}",
                            jni.get_string(unsafe {
                                &jni::objects::JString::from_raw(cause_obj.as_jni().l)
                            })?
                            .to_string_lossy()
                            .to_string()
                        )
                        .as_str();
                    };
                    return Err(eyre::eyre!(message).into());
                } else {
                    Err(err.into())
                }
            }
        }
    }

    pub fn validate_name<O>(
        &self,
        obj: O,
        expected_name: &str,
    ) -> Result<(bool, String), Box<dyn std::error::Error>>
    where
        O: AsRef<jni::objects::JObject<'mc>>,
    {
        let cls = self.get_object_class(&obj)?;
        let name_raw = self.call_method(&cls, "getName", "()Ljava/lang/String;", &[]);
        let name_raw = self.translate_error(name_raw)?;
        let oh = name_raw.l()?.into();
        let what = self.get_string(&oh)?;
        let name = what.to_string_lossy().to_string();
        if !name.ends_with(expected_name) {
            // What about the super class?
            let super_class = self.call_method(&cls, "getSuperclass", "()Ljava/lang/Class;", &[]);
            let super_class = self.translate_error(super_class)?.l().unwrap();
            if super_class.is_null() {
                return Ok((false, name));
            }
            let name_raw = self.call_method(super_class, "getName", "()Ljava/lang/String;", &[]);
            let name_raw = self.translate_error(name_raw)?;
            let oh = name_raw.l()?.into();
            let what = self.get_string(&oh)?;
            let name = what.to_string_lossy().to_string();
            if !name.ends_with(expected_name) {
                Ok((false, name))
            } else {
                Ok((true, name))
            }
        } else {
            Ok((true, name))
        }
    }
}

impl<'mc> Clone for SharedJNIEnv<'mc> {
    fn clone(&self) -> Self {
        Self {
            jni: RefCell::new(
                // The reason it's unsafe is actually OK for this use case. The "local reference frame" is a discrete feature of the library that is only ever used if you instantiate the JVM from the Rust program itself, which we never do. Since the JNI pointer lasts for the lifetime of the program, we can safely clone it.
                unsafe { self.jni.borrow().unsafe_clone() },
            ),
        }
    }
}

pub trait JNIRaw<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc>;
    fn jni_object(&self) -> jni::objects::JObject<'mc>;
}
