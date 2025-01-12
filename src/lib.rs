
/// Mutable Vec<_> init
#[macro_export]
macro_rules! vi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<$e> = vec![];
    };
}

/// Mutable Vec<Vec<_>> init
#[macro_export]
macro_rules! vvi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<Vec<$e>> = vec![];
    };
}

///  Vec<Vec<_>>  type
#[macro_export]
macro_rules! vvt {
    ($e:ty) => {
        Vec<Vec<$e>>
    };
}

/// Vec<_> type
#[macro_export]
macro_rules! vt {
    ($e:ty) => {
        Vec<$e>
    };
}

/// for enumerate
#[macro_export]

macro_rules! for_enum {
    ($ind:ident, $elem:ident, $collection:expr, $body:block) => {
        for ($ind, $elem) in $collection.iter().enumerate() {
            $body
        }
    };
}

