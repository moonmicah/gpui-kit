#[cfg(feature = "gpui-fast")]
extern crate gpui_fast as gpui;
#[cfg(feature = "gpui-fast")]
extern crate gpui_fast_platform as gpui_platform;

#[path = "../../../showcase/mod.rs"]
mod showcase;


fn main() {
    let component = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "overview".to_string());

    #[cfg(not(feature = "gpui-fast"))]
    let app = {
        let http_client =
            reqwest_client::ReqwestClient::user_agent("gpui-base/examples").unwrap();
        gpui_platform::application().with_http_client(Arc::new(http_client))
    };
    #[cfg(feature = "gpui-fast")]
    let app = gpui_platform::application();
    showcase::run(app, component);
}
