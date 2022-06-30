# yew-router-qs

Attempts to route based on a query string.

Static web sites such as IPFS cannot dynamically process routes (unless navigating within an already loaded Yew app), but a static html 404 page can be used to redirect to the default index page, thereby passing the required request details via the query string to Yew.

An example ipfs-404.html redirect page:
```
<head>
    <title>404</title>
    <script type="text/javascript">window.location.href = "/?" + window.location.pathname + window.location.search + window.location.hash;</script>
    <noscript><meta http-equiv="Refresh" content="0; url='/404'" /></noscript>
</head>
```

You can then use `yew_router_qs::try_route_from_query_string` in the `create()` function of the main Yew component, which will attempt to route based on the received query string parameter.

```
fn create(ctx: &Context<Self>) -> Self {
   if let Err(e) = yew_router_qs::try_route_from_query_string() {
        error!(e)
    }
    Self { .. }
}
```

More info on Yew routing at https://yew.rs/docs/concepts/router.
