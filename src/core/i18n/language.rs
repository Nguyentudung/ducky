use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

const EN_TRANSLATIONS: &str = include_str!("../../../extensions/locales/en/app.ftl");

const VI_TRANSLATIONS: &str = include_str!("../../../extensions/locales/vi/app.ftl");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLanguage {
    System,
    English,
    Vietnamese,
}

impl AppLanguage {
    pub fn locale(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::English => "en",
            Self::Vietnamese => "vi",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::System => "System Default",
            Self::English => "English",
            Self::Vietnamese => "Tiếng Việt",
        }
    }
}

pub struct LanguageManager {
    language: AppLanguage,
    bundle: FluentBundle<FluentResource>,
}

impl LanguageManager {
    pub fn new(language: AppLanguage) -> Self {
        Self {
            language,
            bundle: Self::create_bundle(language),
        }
    }

    fn create_bundle(language: AppLanguage) -> FluentBundle<FluentResource> {
        // Resolve the actual locale
        let locale = Self::resolve_locale(language);

        let lang_id: LanguageIdentifier = locale.parse().expect("Invalid language identifier");

        let mut bundle = FluentBundle::new(vec![lang_id]);

        // Choose translation file based on the resolved locale
        let source = match locale {
            "vi" => VI_TRANSLATIONS,
            _ => EN_TRANSLATIONS,
        };

        // Load Fluent resource
        let resource = FluentResource::try_new(source.to_string())
            .expect("Failed to parse Fluent translations");

        bundle
            .add_resource(resource)
            .expect("Failed to add Fluent resource");

        bundle
    }

    fn resolve_locale(language: AppLanguage) -> &'static str {
        match language {
            AppLanguage::English => "en",

            AppLanguage::Vietnamese => "vi",

            AppLanguage::System => {
                let system_locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());

                if system_locale.to_lowercase().starts_with("vi") {
                    "vi"
                } else {
                    "en"
                }
            }
        }
    }

    // Change language at runtime
    pub fn set_language(&mut self, language: AppLanguage) {
        self.language = language;
        self.bundle = Self::create_bundle(language);
    }

    // Get current selected language
    pub fn language(&self) -> AppLanguage {
        self.language
    }

    // Translate a key
    pub fn translate(&self, key: &str) -> String {
        let Some(message) = self.bundle.get_message(key) else {
            return key.to_string();
        };

        let Some(pattern) = message.value() else {
            return key.to_string();
        };

        let mut errors = Vec::new();

        self.bundle
            .format_pattern(pattern, None, &mut errors)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{AppLanguage, LanguageManager};

    #[test]
    fn switches_between_english_and_vietnamese() {
        let mut language = LanguageManager::new(AppLanguage::English);
        assert_eq!(language.translate("navigation-home"), "Home");
        language.set_language(AppLanguage::Vietnamese);
        assert_eq!(language.translate("navigation-home"), "Trang chủ");
        assert_eq!(language.translate("status-solved"), "Đã giải");
    }
}
