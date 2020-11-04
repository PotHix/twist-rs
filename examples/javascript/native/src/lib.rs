use neon::register_module;
use twist_rs::endpoints::search;

use neon::prelude::*;

fn search(mut cx: FunctionContext) -> JsResult<JsString> {
    let token = cx.argument::<JsString>(0)?.value();
    let query = cx.argument::<JsString>(1)?.value();

    let req = search::search(token, query).unwrap();
    Ok(cx.string(req.text().unwrap()))
}

register_module!(mut m, {
    m.export_function("search", search)?;
    Ok(())
});
