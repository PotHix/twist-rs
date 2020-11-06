use twist_rs::endpoints::search;

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Twist_search(
    env: JNIEnv,
    _class: JClass,
    token: JString,
    query: JString,
) -> jstring {
    let token_str: String = env
        .get_string(token)
        .expect("Couldn't get the token")
        .into();

    let query_str: String = env
        .get_string(query)
        .expect("Couldn't get the query")
        .into();

    let req = search::search(token_str, query_str).unwrap();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(req.text().unwrap())
        .expect("Couldn't create java string!");

    // extract the raw pointer to return.
    output.into_inner()
}
