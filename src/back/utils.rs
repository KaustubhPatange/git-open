use crate::NULL;

pub fn launch_github_uri(url: String, branch: &str, path: &str) -> Result<(), String> {
    let mut github_url = url.clone();
    if github_url.ends_with(".git") {
        github_url = github_url.replace(".git","")
    }
    if branch != NULL {
        github_url.push_str("/tree/");
        github_url.push_str(branch)
    }
    if path != NULL {
        if !github_url.contains("/tree/") {
            github_url.push_str("/tree/");
            github_url.push_str("HEAD");
        }
        github_url.push_str("/");
        github_url.push_str(path);
    }

    return if webbrowser::open(github_url.as_str()).is_ok() {
        Ok(())
    } else {
        Err("Unable to launch default browser".to_owned())
    }
}