use std::any::Any;

// see: https://api.flutter.dev/flutter/widgets/RouteSettings-class.html
/*
pub struct RouteSettings {
    name: String,
    arguments: Vec<(String, String)>, // vec of key/value strings ?
}
*/

pub struct Route {
    // name of the route, ie "/settings"
    name: String,
    /*

    // Whether this route is on the navigator
    is_active: bool,

    // Whether this route is the top-most route on the navigator
    is_current: bool,

    // Whether this route is the bottom-most route on the navigator
    is_first: bool,

    // The navigator that the route is in, if any
    navigator: Option<Navigator>, // TODO: ref to static lifetime navigator?

    */
}

pub struct Navigator {}
