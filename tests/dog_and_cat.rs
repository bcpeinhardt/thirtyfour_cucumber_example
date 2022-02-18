use async_trait::async_trait;
use cucumber::{given, then, when, WorldInit};
use thirtyfour::error::WebDriverError;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use thirtyfour_cucumber_example::{NavigateToWikipedia, WikipediaHomePage};

// The cucumber crate has a trait "World" which you can implement on your own 
// struct to use as a way to share context between your tests. I'll
// use it to initialize and pass my WebDriver from step to step.
#[derive(Debug, WorldInit)]
struct Context {
    driver: WebDriver,
}

#[async_trait(?Send)]
impl cucumber::World for Context {
    type Error = WebDriverError;

    async fn new() -> Result<Self, Self::Error> {

        // Cucmber features are executed concurrently by default, so if your tests involve
        // Selenium, you'll want to use Selenium Grid for multiple WebDrivers. 
        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;
        Ok(Self { driver })
    }
}

#[given("browser successfully navigates to wikipedia")]
async fn navigate_to_wikipedia(ctx: &mut Context) -> WebDriverResult<()> {
    ctx.driver.navigate_to_wikipedia().await?;
    Ok(())
}

#[when(expr = "I search for {word}")]
async fn search_for(ctx: &mut Context, search_text: String) -> WebDriverResult<()> {
    WikipediaHomePage::from_driver_ref(&ctx.driver)
        .await?
        .search_for(&search_text)
        .await?;
    Ok(())
}

#[then(expr = "browser navigates to {word}")]
async fn browser_navigates_to(ctx: &mut Context, url: String) -> WebDriverResult<()> {

    // Wait a few seconds for the URL to redirect. Obviously don't do random sleeps in production tests
    sleep(Duration::from_secs(5)).await;
    assert_eq!(
        url,
        ctx.driver.current_url().await?
    );
    ctx.driver.close().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    Context::run("tests/features").await;
}
