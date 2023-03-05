use serde::{de::{self, Visitor, Expected}, Deserialize, Deserializer};
use serde_json::Value;
use std::{marker::PhantomData, fmt::Debug};



#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
enum Style {
    FontSize(f64),
    Bold(bool),
    Align(String)
}

type Styles = std::collections::HashMap<String, Style>;





fn de_style<'de, D>(deserializer: D) -> Result<Styles, D::Error>
    where 
        D: Deserializer<'de>,
{ 
    

    struct StylesVisitor<K, V> {
            marker: PhantomData<fn() -> std::collections::HashMap<K, V>>
    }

    impl<K,V> StylesVisitor<K,V> {
        fn new() -> Self {
            Self {marker: PhantomData}
        }
    }



    impl<'de, K,V> Visitor<'de> for StylesVisitor<K,V>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,

    {
        type Value = Styles;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Style Attributes")
        }

        fn visit_map<D>(self, mut access: D) -> Result<Self::Value, D::Error>
            where
                D: de::MapAccess<'de>, 
            {
                let mut map: Styles = std::collections::HashMap::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((key, value)) = access.next_entry::<&str, Value>()? {
                    match key {
                        "fontSize" => {
                            if let Some(v) = value.as_f64() {
                                map.insert(key.to_string(), Style::FontSize(v));
                            } else {
                                return Err(de::Error::custom("Expected f64"))
                            }
                        },
                        "bold" => {
                            if let Some(v) = value.as_bool() {
                                map.insert(key.to_string(), Style::Bold(v));
                            } else {
                                return Err(de::Error::custom("Expected bool"))
                            }
                        },
                        "align" => {
                            if let Some(v) = value.as_str() {
                                map.insert(key.to_string(), Style::Align(v.to_string()));
                            } else {
                                return Err(de::Error::custom("Expected String"))
                            }
                        }
                        _ => unreachable!("Unknown Style")
                    }
                }
                Ok(map)
        }
    }
    let v: StylesVisitor<String, String> = StylesVisitor::new();
    deserializer.deserialize_map(v)
}


#[cfg(test)]
mod test {
    use super::*;
    #[derive(Debug, Deserialize, PartialEq)]
    pub struct StyleWrapper {
        #[serde(deserialize_with = "de_style")]
        style: Styles
    }
    #[test]
    fn test_style_deserialization() {
        let data = r#"
         {
            "style": {
                "fontSize": 12.0,
                "bold": true,
                "align": "right"
            }
        }"#;
        let result: StyleWrapper  = serde_json::from_str(data).unwrap();
        let mut expect = StyleWrapper {
            style: Styles::new()
        };
        expect.style.insert("fontSize".to_string(), Style::FontSize(12.0));
        expect.style.insert("bold".to_string(), Style::Bold(true));
        expect.style.insert("align".to_string(), Style::Align("right".to_string()));

        assert_eq!(result, expect);
    }

}
