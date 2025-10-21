use std::collections::HashMap;
use std::sync::RwLock;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Locale {
    En,
    Es,
}

impl Locale {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Some(Locale::En),
            "es" | "spanish" | "español" => Some(Locale::Es),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Es => "es",
        }
    }

    fn file_name(&self) -> &'static str {
        match self {
            Locale::En => "locales/en.yaml",
            Locale::Es => "locales/es.yaml",
        }
    }
}

fn load_translations_from_file(locale: Locale) -> HashMap<String, String> {
    let file_path = locale.file_name();
    let mut translations = HashMap::new();

    if let Ok(content) = std::fs::read_to_string(file_path) {
        if let Ok(docs) = YamlLoader::load_from_str(&content) {
            if let Some(Yaml::Hash(hash)) = docs.get(0) {
                for (key, value) in hash {
                    if let (Yaml::String(key_str), Yaml::String(value_str)) = (key, value) {
                        translations.insert(key_str.clone(), value_str.clone());
                    }
                }
            }
        }
    }

    translations
}

lazy_static::lazy_static! {
    static ref CURRENT_LOCALE: RwLock<Locale> = RwLock::new(Locale::En);
    static ref TRANSLATIONS: HashMap<(String, Locale), String> = {
        let mut m = HashMap::new();

        // Load English translations
        let en_translations = load_translations_from_file(Locale::En);
        for (key, value) in en_translations {
            m.insert((key.clone(), Locale::En), value);
        }

        // Load Spanish translations
        let es_translations = load_translations_from_file(Locale::Es);
        for (key, value) in es_translations {
            m.insert((key.clone(), Locale::Es), value);
        }

        m
    };
}

pub fn set_locale(locale: Locale) {
    *CURRENT_LOCALE.write().unwrap() = locale;
}

pub fn get_locale() -> Locale {
    *CURRENT_LOCALE.read().unwrap()
}

pub fn t(key: &str) -> String {
    let locale = get_locale();
    TRANSLATIONS
        .get(&(key.to_string(), locale))
        .unwrap_or(&key.to_string())
        .clone()
}

pub fn t_with_args(key: &str, args: &[&str]) -> String {
    let template = t(key);
    let mut result = template;

    // Replace indexed placeholders {0}, {1}, etc.
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }

    // Also handle single {} placeholder for backward compatibility
    if args.len() == 1 && result.contains("{}") {
        result = result.replace("{}", args[0]);
    }

    result
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::t($key)
    };
    ($key:expr, $($arg:expr),*) => {
        $crate::i18n::t_with_args($key, &[$($arg),*])
    };
}
