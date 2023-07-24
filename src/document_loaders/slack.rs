use super::*;
use slack_morphism::prelude::*;

// # SlackLoader
//
// channelがthreadの場合は、channelとtsをハイフンでつなげた文字列をchannelに指定する
// 例: C017Y386TNK-1690187559.790209
//
// channelがthreadでない場合は、channelを指定する
// 例: C017Y386TNK
pub struct SlackLoader<'a> {
    token: &'a str,
    channel: &'a str,
    limit: u16,
    cursor: Option<String>,
}

impl<'a> SlackLoader<'a> {
    pub fn new(
        token: &'a str,
        channel: &'a str,
        limit: Option<u16>,
        cursor: Option<String>,
    ) -> Self {
        let limit = limit.unwrap_or(100);
        Self {
            token,
            channel,
            limit,
            cursor,
        }
    }
}

#[async_trait::async_trait]
impl<'a> Loader for SlackLoader<'a> {
    async fn load(&self) -> anyhow::Result<Vec<Document>> {
        let connector = SlackClientHyperConnector::new();
        let client = SlackClient::new(connector);

        let token_value = SlackApiToken {
            token_value: self.token.into(),
            team_id: None,
            scope: None,
            token_type: None,
        };
        let session = client.open_session(&token_value);
        let places = self.channel.split('-').collect::<Vec<_>>();
        if places.len() != 1 {
            // thread
            let reply = session
                .conversations_replies(&SlackApiConversationsRepliesRequest {
                    channel: places[0].into(),
                    ts: places[1].into(),
                    cursor: self.cursor.clone().map(|c| c.into()),
                    limit: Some(self.limit),
                    latest: None,
                    oldest: None,
                    inclusive: None,
                })
                .await?;
            return Ok(reply
                .messages
                .into_iter()
                .enumerate()
                .map(|(index, message)| {
                    Document::new(&message.content.text.unwrap_or("".into()), index)
                })
                .collect());
        };
        let reply = session
            .conversations_history(&SlackApiConversationsHistoryRequest {
                channel: Some(places[0].into()),
                cursor: self.cursor.clone().map(|c| c.into()),
                limit: Some(self.limit),
                latest: None,
                oldest: None,
                inclusive: None,
            })
            .await?;
        Ok(reply
            .messages
            .into_iter()
            .enumerate()
            .map(|(index, message)| {
                Document::new(&message.content.text.unwrap_or("".into()), index)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_env_var(name: &str) -> anyhow::Result<String> {
        Ok(std::env::var(name)?)
    }

    #[tokio::test]
    async fn test_document_load_slack() -> anyhow::Result<()> {
        let token = config_env_var("SLACK_TOKEN")?;
        let loader = SlackLoader::new(&token, "C017Y386TNK-1690187559.790209", None, None);
        let docs = loader.load().await?;
        dbg!(docs);

        Ok(())
    }
}
