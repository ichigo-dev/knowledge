use crate::api::GitHubApi;
use select::document::Document;
use select::predicate::{ Name, Predicate };

#[derive(Debug)]
pub struct Term
{
    term: String,
    path: String,
}

impl Term
{
    pub fn new( term: String, path: String ) -> Self
    {
        Term
        {
            term,
            path,
        }
    }

    pub fn term( &self ) -> &str
    {
        self.term.as_str()
    }

    pub fn path( &self ) -> &str
    {
        self.path.as_str()
    }

    pub async fn generate_quiz( &self, lang_code: &str ) -> String
    {
        let flagment = if self.path.contains('#')
        {
            let split = self.path.split_once('#').unwrap();
            urlencoding::decode(split.1).unwrap().into_owned()
        }
        else
        {
            "".to_string()
        };

        //  Gets the content from GitHub.
        let api = GitHubApi::new();
        let content = api.get_content(lang_code, &self.path).await;

        //  Generates a quiz from the content.
        let quiz = if flagment.is_empty()
        {
            content
        }
        else
        {
            let doc = Document::from(content.as_str());

            let mut quiz = String::new();
            for h in doc.find(Name("h2").or(Name("h3")).or(Name("h4")))
            {
                if h.text() == flagment
                {
                    let mut node = h;
                    while node.next().is_some()
                    {
                        if node.next().unwrap().name() == Some("h2")
                            || node.next().unwrap().name() == Some("h3")
                            || node.next().unwrap().name() == Some("h4")
                        {
                            break;
                        }
                        quiz += node.next().unwrap().inner_html().as_str();
                        node = node.next().unwrap();
                    }
                }
            }
            quiz
        };

        //  Masks the quiz.
        let quiz = quiz.replace(flagment.as_str(), "__________");
        let quiz = quiz.replace(&self.term, "__________");

        quiz
    }
}
