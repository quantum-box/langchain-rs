use super::*;
use headless_chrome::{Browser, LaunchOptions};

pub struct BrowserURLLoader<'a> {
    urls: Vec<&'a str>,
    continue_on_failure: bool,
    browser: BrowserKind,
    binary_location: Option<&'a str>,
    executable_path: Option<&'a str>,
    headless: bool,
    arguments: Vec<&'a str>,
}

impl<'a> BrowserURLLoader<'a> {
    pub fn new(
        urls: Vec<&'a str>,
        continue_on_failure: bool,
        browser: BrowserKind,
        binary_location: Option<&'a str>,
        executable_path: Option<&'a str>,
        headless: bool,
        arguments: Vec<&'a str>,
    ) -> Self {
        Self {
            urls,
            continue_on_failure,
            browser,
            binary_location,
            executable_path,
            headless,
            arguments,
        }
    }

    pub fn default(url: &'a str) -> Self {
        Self {
            urls: vec![url],
            continue_on_failure: false,
            browser: BrowserKind::Chrome,
            binary_location: None,
            executable_path: None,
            headless: true,
            arguments: vec![],
        }
    }

    fn get_browser(&self) -> anyhow::Result<Browser> {
        Ok(Browser::new(LaunchOptions {
            headless: self.headless,
            ..Default::default()
        })?)
    }
}

#[async_trait::async_trait]
impl<'a> Loader for BrowserURLLoader<'a> {
    async fn load(&self) -> anyhow::Result<Vec<Document>> {
        let mut docs: Vec<Document> = Vec::new();

        let browser = self.get_browser()?;
        for (index, url) in self.urls.iter().enumerate() {
            let tab = browser.new_tab()?;
            tab.set_default_timeout(std::time::Duration::from_secs(200));
            tab.navigate_to(url)?;
            tab.wait_until_navigated()?;
            // TODO: ここは後で直す bodyをとりたい
            let text = tab.wait_for_element("body")?.get_content()?;
            docs.push(Document {
                page_content: text,
                lookup_str: "".to_string(),
                lookup_index: index,
                metadata: None,
            });
        }
        Ok(docs)
    }
}

pub enum BrowserKind {
    Chrome,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_document_load_browser() -> anyhow::Result<()> {
        let loader = BrowserURLLoader::new(
            vec![&"https://www.google.com"],
            false,
            BrowserKind::Chrome,
            None,
            None,
            false,
            vec![],
        );
        let docs = loader.load().await?;
        assert_eq!(docs.len(), 1);
        dbg!(&docs);
        Ok(())
    }
}
