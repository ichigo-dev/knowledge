use reqwest;

pub struct GitHubApi
{
    client: reqwest::Client,
}

impl GitHubApi
{
    const API_ENDPOINT: &str = "https://api.github.com";
    const API_TOKEN: &str = "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";
    const API_OWNER: &str = "ichigo-dev";
    const API_REPO: &str = "knowledge";

    pub fn new() -> Self
    {
        GitHubApi
        {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_content( &self, path: &str ) -> Option<String>
    {
        let url = format!
        (
            "{}/repos/{}/{}/contents/{}",
            Self::API_ENDPOINT,
            Self::API_OWNER,
            Self::API_REPO,
            path
        );
        let response = self.client
            .get(url)
            .bearer_auth(Self::API_TOKEN)
            .header("Accept", "application/vnd.github.html")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .unwrap();

        if response.status().is_success()
        {
            let body = response
                .text()
                .await
                .unwrap_or("".to_string());
            log::info!("{:?}", body);
            return Some(body);
        }

        None
    }
}
