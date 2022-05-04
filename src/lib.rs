use wasm_bindgen::JsValue;
use web_sys::Window;

/// Attempts to route based on a query string.
///
/// # Examples
/// Static web sites cannot process routes, but a static 404 page can be used to redirect to the default index page,
/// passing the required request details via the query string.
///
/// ```html
/// <head>
///     <title>404</title>
///     <script type="text/javascript">window.location.href = "/?" + window.location.pathname + window.location.search + window.location.hash;</script>
///     <noscript><meta http-equiv="Refresh" content="0; url='/404'" /></noscript>
///  </head>
/// ```
/// [try_route_from_query_string] can then be called during the [create] method of your main Yew component,
/// which replaces the current window history state with the requested location, allowing Yew to match on the route.
///
/// ```text
/// fn create(ctx: &Context<Self>) -> Self {
///     if let Err(e) = yew_router_qs::try_route_from_query_string() {
///         error!(e)
///     }
///     Self { .. }
/// }
/// ```
pub fn try_route_from_query_string() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("could not find window");
    let search = window.location().search()?;
    if search.len() == 0 {
        return Ok(());
    }

    window
        .history()?
        .replace_state_with_url(&JsValue::NULL, "", Some(&search[1..].to_string()))
}
