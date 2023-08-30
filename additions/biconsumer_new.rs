// blackboxmc-rs-java/src/util/function/mod.rs#JavaBiConsumer

#[cfg(not(target_arch = "wasm32"))]
pub fn from_rust_fn<'mc2, A, B, C>(
    env: &'mc2 blackboxmc_general::SharedJNIEnv<'mc>,
    f: A,
) -> Result<Self, Box<dyn std::error::Error>>
where
    A: Fn(&'mc2 blackboxmc_general::SharedJNIEnv<'mc>, B, C),
    B: blackboxmc_general::JNIInstantiatable<'mc>,
    C: blackboxmc_general::JNIInstantiatable<'mc>,
{
    // Create the closure.
    let closure: &_ = Box::leak(Box::new(
        move |a: *mut jni::sys::_jobject, b: *mut jni::sys::_jobject| unsafe {
            f(
                env,
                B::from_raw(&env, jni::objects::JObject::from_raw(a)).unwrap(),
                C::from_raw(&env, jni::objects::JObject::from_raw(b)).unwrap(),
            );
        },
    ));
    let callback = libffi::high::Closure2::new(closure);
    let &code = callback.code_ptr();
    let ptr: unsafe extern "C" fn(*const jni::sys::jobject) = unsafe { std::mem::transmute(code) };
    std::mem::forget(callback);

    // Create the java object.
    let obj = env.new_object(
        "net/ioixd/blackbox/BiConsumerLink",
        "(J)V",
        vec![(ptr as i64).into()],
    );
    let obj = env.translate_error_no_gen(obj)?;

    Self::from_raw(&env, obj)
}
