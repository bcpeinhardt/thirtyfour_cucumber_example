use async_trait::async_trait;
use thirtyfour::error::WebDriverError;
use thirtyfour::prelude::*;

#[async_trait]
pub trait NavigateToWikipedia {
    type Error;
    async fn navigate_to_wikipedia(&self) -> Result<(), <Self as NavigateToWikipedia>::Error>;
}

#[async_trait]
impl NavigateToWikipedia for WebDriver {
    type Error = WebDriverError;
    async fn navigate_to_wikipedia(&self) -> WebDriverResult<()> {
        self.get("https://www.wikipedia.org/").await?;
        Ok(())
    }
}

pub struct WikipediaHomePage<'a> {
    driver_ref: &'a WebDriver,
}

impl<'a> WikipediaHomePage<'a> {
    pub async fn from_driver_ref(
        driver_ref: &'a WebDriver,
    ) -> WebDriverResult<WikipediaHomePage<'a>> {
        driver_ref.navigate_to_wikipedia().await?;
        Ok(WikipediaHomePage { driver_ref })
    }

    pub async fn search_for(self, input: &str) -> WebDriverResult<WikipediaContentPage<'a>> {
        let search_bar = self.driver_ref.query(By::Id("searchInput")).first().await?;

        search_bar.send_keys(input).await?;
        search_bar.send_keys(Keys::Enter).await?;

        Ok(WikipediaContentPage {
            driver_ref: self.driver_ref,
        })
    }
}

pub struct WikipediaContentPage<'a> {
    driver_ref: &'a WebDriver,
}
