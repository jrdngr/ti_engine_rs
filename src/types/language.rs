use std::str::FromStr;

// This derive statement automatically implements the traits in parentheses.
// This works for any built-in trait with an obvious implementation.
// You an also make your own traits derivable, but that's a little complex.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Language {
    en_US,
    fe_FR,
    es_ES,
    de_DE,
    ja_JP,
}

// FromStr is a trait that defines how you parse your type from a string
impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "en_US" => Ok(Language::en_US),
            "fe_FR" => Ok(Language::fe_FR),
            "es_ES" => Ok(Language::es_ES),
            "de_DE" => Ok(Language::de_DE),
            "ja_JP" => Ok(Language::ja_JP),
            _ => anyhow::bail!("Invalid input: {}", string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_as_int() {
        assert_eq!(Language::en_US as u32, 0);
        assert_eq!(Language::fe_FR as u32, 1);
        assert_eq!(Language::es_ES as u32, 2);
        assert_eq!(Language::de_DE as u32, 3);
        assert_eq!(Language::ja_JP as u32, 4);
    }

    #[test]
    fn language_as_string_valid() {
        assert_eq!(Language::from_str("en_US").unwrap(), Language::en_US);
        assert_eq!(Language::from_str("fe_FR").unwrap(), Language::fe_FR);
        assert_eq!(Language::from_str("es_ES").unwrap(), Language::es_ES);
        assert_eq!(Language::from_str("de_DE").unwrap(), Language::de_DE);
        assert_eq!(Language::from_str("ja_JP").unwrap(), Language::ja_JP);
    }

    #[test]
    fn language_as_string_invalid() {
        assert!(Language::from_str("oh noes").is_err());
        assert!(Language::from_str("kl_KL").is_err());
    }
}
