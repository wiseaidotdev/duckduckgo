use duckduckgo::browser::Browser;

#[test]
fn test_browser_new_creates_instance() {
    let _browser = Browser::new();
}

#[test]
fn test_browser_default_equals_new() {
    let _browser: Browser = Browser::default();
}

#[test]
fn test_browser_builder_default_builds_successfully() {
    let browser = Browser::builder().build();
    assert!(
        browser.is_ok(),
        "BrowserBuilder::default().build() should succeed"
    );
}

#[test]
fn test_browser_builder_with_user_agent() {
    let browser = Browser::builder().user_agent("Mozilla/5.0 (Test)").build();
    assert!(browser.is_ok(), "Builder with user_agent should succeed");
}

#[test]
fn test_browser_builder_with_cookie_store_enabled() {
    let browser = Browser::builder().cookie_store(true).build();
    assert!(
        browser.is_ok(),
        "Builder with cookie_store(true) should succeed"
    );
}

#[test]
fn test_browser_builder_with_cookie_store_disabled() {
    let browser = Browser::builder().cookie_store(false).build();
    assert!(
        browser.is_ok(),
        "Builder with cookie_store(false) should succeed"
    );
}

#[test]
fn test_browser_builder_with_valid_proxy() {
    let browser = Browser::builder().proxy("http://127.0.0.1:8080").build();
    assert!(
        browser.is_ok(),
        "Builder with a valid proxy URL should succeed"
    );
}

#[test]
fn test_browser_builder_with_all_options() {
    let browser = Browser::builder()
        .user_agent("TestAgent/1.0")
        .cookie_store(true)
        .proxy("http://127.0.0.1:3128")
        .build();
    assert!(
        browser.is_ok(),
        "Builder with all options set should succeed"
    );
}
