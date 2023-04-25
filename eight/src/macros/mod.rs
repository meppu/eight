/// Easily create [`Request`].
///
/// ```
/// use eight::messaging::Request;
///
/// assert_eq!(Request::Flush, eight::request!(Flush));
/// assert_eq!(Request::Get("test".into()), eight::request!(Get, "test"));
/// assert_eq!(Request::Set("test".into(), 5.to_string()), eight::request!(Set, "test", 5));
/// ```
///
/// [`Request`]: ./enum.Request.html
#[macro_export]
macro_rules! request {
    ($name:ident) => {
        eight::messaging::Request::$name;
    };

    ($name:ident, $($value:expr),*) => {
        eight::messaging::Request::$name($($value.to_string()),*);
    };
}

/// Easily create [`std::collections::HashMap<String, String>`].
///
/// ```
/// use std::collections::HashMap;
///
/// let mut hard_way = HashMap::new();
///
/// hard_way.insert("key".to_string(), "hello".to_string());
/// hard_way.insert("other".to_string(), (3.14).to_string());
///
/// let easy_way = eight::env!(key: "hello", other: 3.14);
///
/// assert_eq!(hard_way, easy_way);
/// ```
#[macro_export]
macro_rules! env {
    ($($key:ident: $value:expr),*) => {
        {
            let mut env = std::collections::HashMap::<String, String>::new();

            $(
                {
                    env.insert(stringify!($key).to_string(), ($value).to_string());
                }
            )*

            env
        }
    };
}
