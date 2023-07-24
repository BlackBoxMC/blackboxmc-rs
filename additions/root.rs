// src/lib.rs
use std::cell::RefCell;

use jni::{
    descriptors::Desc,
    errors::Error,
    objects::{JObject, JString, JValue},
    strings::{JNIString, JavaStr},
};

pub struct SharedJNIEnv<'mc> {
    jni: RefCell<jni::JNIEnv<'mc>>,
}

impl<'mc> SharedJNIEnv<'mc> {
    pub fn new(jni: jni::JNIEnv<'mc>) -> Self {
        Self {
            jni: RefCell::new(jni),
        }
    }
    pub fn new_string<S: Into<JNIString> + std::convert::AsRef<str>>(
        &self,
        from: S,
    ) -> Result<JString<'mc>, Error> {
        Ok(self.jni.borrow_mut().new_string(from)?)
    }
    pub fn new_object<'other_local, T, U>(
        &self,
        class: T,
        ctor_sig: U,
        ctor_args: &[JValue<'_, '_>],
    ) -> Result<JObject<'mc>, jni::errors::Error>
    where
        T: Desc<'mc, jni::objects::JClass<'other_local>>,
        U: Into<jni::strings::JNIString> + AsRef<str>,
    {
        Ok(self
            .jni
            .borrow_mut()
            .new_object(class, ctor_sig, ctor_args)?)
    }
    pub fn get_string<'other_local: 'obj_ref, 'obj_ref>(
        &self,
        obj: &'obj_ref jni::objects::JString<'other_local>,
    ) -> Result<jni::strings::JavaStr<'mc, 'other_local, 'obj_ref>, jni::errors::Error> {
        Ok(self.jni.borrow_mut().get_string(obj)?)
    }
    pub fn call_method<'other_local, O, S, T>(
        &self,
        obj: O,
        name: S,
        sig: T,
        args: &[jni::objects::JValue<'_, '_>],
    ) -> Result<jni::objects::JValueOwned<'mc>, jni::errors::Error>
    where
        O: AsRef<jni::objects::JObject<'other_local>>,
        S: Into<jni::strings::JNIString>,
        T: Into<jni::strings::JNIString> + AsRef<str>,
    {
        Ok(self.jni.borrow_mut().call_method(obj, name, sig, args)?)
    }
    pub fn find_class<S>(&self, name: S) -> Result<jni::objects::JClass<'mc>, jni::errors::Error>
    where
        S: Into<JNIString> + std::convert::AsRef<str>,
    {
        Ok(self.jni.borrow_mut().find_class(name)?)
    }
    pub fn call_static_method<'other_local, T, U, V>(
        &self,
        class: T,
        name: U,
        sig: V,
        args: &[jni::objects::JValue<'_, '_>],
    ) -> Result<jni::objects::JValueOwned<'mc>, jni::errors::Error>
    where
        T: jni::descriptors::Desc<'mc, jni::objects::JClass<'other_local>>,
        U: Into<jni::strings::JNIString>,
        V: Into<jni::strings::JNIString> + AsRef<str>,
    {
        Ok(self
            .jni
            .borrow_mut()
            .call_static_method(class, name, sig, args)?)
    }

    pub fn exception_occurred(&self) -> Result<jni::objects::JThrowable<'mc>, jni::errors::Error> {
        self.jni.borrow_mut().exception_occurred()
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

/// Function that checks if an exception is being thrown, and turns it into a Rust error.
pub fn java_error_throw<'mc>(
    jni: SharedJNIEnv<'mc>,
    res: Result<jni::objects::JValueGen<jni::objects::JObject<'mc>>, jni::errors::Error>,
) -> Result<jni::objects::JValueGen<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
    match res {
        Ok(res) => {
            Ok(res)
        }
        Err(err) => {
            let exp = jni.exception_occurred()?;
            if !exp.is_null() {
                jni.exception_clear()?;
                let mut message = String::new();
                let obj = jni.call_method(&exp, "getMessage", "()Ljava/lang/String;", &[])?;
                let mut message = format!(
                    "{}",
                    jni.get_string(unsafe { &jni::objects::JString::from_raw(obj.as_jni().l) })?
                        .to_string_lossy()
                        .to_string()
                );
                // Is there a cause?
                let cause = jni.call_method(&exp, "getCause", "()Ljava/lang/Throwable;", &[])?.l().unwrap();
                if !&cause.is_null() {
                    message += "\ncaused by: ";
                    let cause_obj = jni.call_method(&cause, "getMessage", "()Ljava/lang/String;", &[])?;
                    message += format!(
                        "{}",
                        jni.get_string(unsafe { &jni::objects::JString::from_raw(cause_obj.as_jni().l) })?
                            .to_string_lossy()
                            .to_string()
                    ).as_str();
                };
                return Err(eyre::eyre!(message).into());
            } else {
                Err(err.into())
            }
        }
    }
}
