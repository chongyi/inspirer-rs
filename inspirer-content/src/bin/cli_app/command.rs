use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::cli_app::Application;
use inspirer_content::ContentService;
use inspirer_content::model::ContentEntityWritable;

pub async fn create(app: &Application) -> Result<u64, Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();

    let title = rl.readline("Title > ")?;
    let keywords = rl.readline("Keywords > ")?;
    let description = rl.readline("Description > ")?;

    let mut content = String::new();
    loop {
        let input = rl.readline("Content > ");
        match input {
            Ok(result) => content.push_str(&format!("{}\n", result)),
            Err(_) => break,
        }
    }

    let result = app.0.create(0, ContentEntityWritable {
        title: title.as_str(),
        keywords: keywords.as_str(),
        description: description.as_str(),
        content: content.as_str()
    }).await?;

    println!("Content created. id = {}", result);

    Ok(result)
}