
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

/// HashMap<_,_> init
#[macro_export]
macro_rules! hmi {
    ($i:tt, $a:ty, $b:ty) => {
        let mut $i: HashMap<$a, $b> = HashMap::new();
    };
}

/// HashMap<_,_> type
#[macro_export]
macro_rules! hmt {
    ($a:ty, $b:ty) => {
        HashMap<$a, $b>
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

/// read lines from file
#[macro_export]
macro_rules! flines {
    ($fn:tt, $lines:tt) => {
        let f = File::open($fn).expect(&format!("File {} not found", $fn));
        let file = BufReader::new(&f);
        let $lines = file.lines();
    };
}
