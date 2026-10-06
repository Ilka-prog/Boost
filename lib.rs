use jni::errors::Result;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::jint;
use jni::JNIEnv;

const FLAG_SYSTEM: i32 = 1;

#[no_mangle]
pub extern "system" fn Java_com_example_booster_Native_killAll<'l>(
    mut env: JNIEnv<'l>,
    _class: JClass<'l>,
    ctx: JObject<'l>,
    keep: JString<'l>,
) -> jint {
    run(&mut env, &ctx, &keep).unwrap_or(-1)
}

fn string_of(env: &mut JNIEnv, s: JObject) -> Result<String> {
    Ok(env.get_string(&JString::from(s))?.into())
}

fn run(env: &mut JNIEnv, ctx: &JObject, keep: &JString) -> Result<jint> {
    let keep: String = env.get_string(keep)?.into();
    let own_obj = env
        .call_method(ctx, "getPackageName", "()Ljava/lang/String;", &[])?
        .l()?;
    let own = string_of(env, own_obj)?;

    let pm = env
        .call_method(ctx, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[])?
        .l()?;
    let name = env.new_string("activity")?;
    let am = env
        .call_method(
            ctx,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&name)],
        )?
        .l()?;
    let apps = env
        .call_method(&pm, "getInstalledApplications", "(I)Ljava/util/List;", &[JValue::Int(0)])?
        .l()?;
    let n = env.call_method(&apps, "size", "()I", &[])?.i()?;

    let mut killed: jint = 0;
    for i in 0..n {
        let did_kill = env.with_local_frame(8, |env| -> Result<bool> {
            let info = env
                .call_method(&apps, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)])?
                .l()?;
            let flags = env.get_field(&info, "flags", "I")?.i()?;
            let pkg_obj = env
                .get_field(&info, "packageName", "Ljava/lang/String;")?
                .l()?;
            let pkg = string_of(env, pkg_obj)?;
            if flags & FLAG_SYSTEM != 0 || pkg == keep || pkg == own {
                return Ok(false);
            }
            let js = env.new_string(&pkg)?;
            env.call_method(
                &am,
                "killBackgroundProcesses",
                "(Ljava/lang/String;)V",
                &[JValue::Object(&js)],
            )?;
            Ok(true)
        })?;
        if did_kill {
            killed += 1;
        }
    }
    Ok(killed)
}
