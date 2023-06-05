use super::*;

#[derive(Debug)]
pub struct DisplayKeywords {
    text: String,
}

impl Executed for DisplayKeywords {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, theme: JupyterTheme) -> Value {
        let color = match theme {
            JupyterTheme::Light => "#A626A4",
            JupyterTheme::Dark => "#A626A4",
        };
        Value::String(format!(r#"<span style="color: {color}">{}</span>"#, self.text))
    }
}

impl DisplayKeywords {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string() }
    }
}

#[derive(Debug)]
pub struct DisplayText {
    text: String,
}

impl Executed for DisplayText {
    fn mime_type(&self) -> String {
        "text/plaintext".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.text.clone())
    }
}

#[derive(Debug)]
pub struct DisplayNumber {
    r#type: String,
    text: String,
}

impl Executed for DisplayNumber {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, theme: JupyterTheme) -> Value {
        let color = match theme {
            JupyterTheme::Light => "#986801",
            JupyterTheme::Dark => "#986801",
        };
        Value::String(format!(r#"<span style="color: {color}">{}</span>"#, self.text))
    }
}

impl DisplayNumber {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { r#type: String::new(), text: text.to_string() }
    }
    pub fn typed<T, S>(text: T, r#type: S) -> Self
        where
            T: ToString,
            S: ToString,
    {
        Self { r#type: r#type.to_string(), text: text.to_string() }
    }
}
