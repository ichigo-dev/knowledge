use crate::term::Term;

use reqwest;
use select::document::Document;
use select::predicate::Name;
use rand::seq::SliceRandom;

pub struct GitHubApi
{
    client: reqwest::Client,
}

impl GitHubApi
{
    const API_ENDPOINT: &str = "https://api.github.com";
    const API_OWNER: &str = "ichigo-dev";
    const API_REPO: &str = "knowledge";
    const API_TOKEN: &str = "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";

    pub fn new() -> Self
    {
        GitHubApi
        {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_terms( &self, lang_code: &str ) -> Vec<Term>
    {
        let url = format!
        (
            "{}/repos/{}/{}/contents/note/{}/checksheet.md",
            Self::API_ENDPOINT,
            Self::API_OWNER,
            Self::API_REPO,
            lang_code,
        );
        let response = self.client
            .get(url)
            .header("Accept", "application/vnd.github.html")
            .bearer_auth(Self::API_TOKEN)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let doc = Document::from(response.as_str());
        let mut terms = Vec::new();
        for li in doc.find(Name("li"))
        {
            let a = li.find(Name("a")).next().unwrap();
            let term = a.text();
            let path = a.attr("href").unwrap_or("").to_string();
            if term.len() <= 0 || path.len() <= 2
            {
                continue;
            }
            terms.push(Term::new(term, path[2..].to_string()));
        }

        let mut rng = rand::thread_rng();
        terms.shuffle(&mut rng);
        terms
    }

    pub async fn get_content( &self, path: &str ) -> String
    {
        let url = format!
        (
            "{}/repos/{}/{}/contents/note/{}",
            Self::API_ENDPOINT,
            Self::API_OWNER,
            Self::API_REPO,
            path,
        );
        let response = self.client
            .get(url)
            .header("Accept", "application/vnd.github.html")
            .bearer_auth(Self::API_TOKEN)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        response
    }
}
