#[cfg(test)]
mod tests;

pub mod searcher {
    pub fn search(text: &str, file: &str) {
        eprintln!("Looking for \"{}\" in file \"{}\"", text, file);
    }
}