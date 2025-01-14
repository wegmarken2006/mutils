
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

/// For enumerate
/// ```
///    for_enum!(ind, elem, vector, {
///        println!("{} {}", ind, elem);
///    });
/// ```
#[macro_export]
macro_rules! for_enum {
    ($ind:ident, $elem:ident, $collection:expr, $body:block) => {
        for ($ind, $elem) in $collection.iter().enumerate() {
            $body
        }
    };
}

/// Read lines from file
#[macro_export]
macro_rules! flines {
    ($fn:tt, $lines:tt) => {
        let f = File::open($fn).expect(&format!("File {} not found", $fn));
        let file = BufReader::new(&f);
        let $lines = file.lines();
    };
}

/// File walk
/// ```
///    fwalk!("/", "xml", file_path, {
///        println!("{}", file_path);
///    });
/// ```

#[macro_export]
macro_rules! fwalk {
    ($sdir:tt, $ext:tt, $fpath:tt, $block:expr) => {
        let root_path = Path::new($sdir);
        if root_path.is_dir() {
            match fs::read_dir(root_path) {
                Ok(dirs) => {
                    for entry in dirs {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_dir() {
                            all_files(path.to_str().unwrap(), $ext);
                        } else if path.extension().unwrap_or_default() == $ext || $ext == "*" {
                            let $fpath = format!("{}", path.display());
                            $block
                        }
                    }
                }
                Err(_) => println!("No permissions for: {}", root_path.to_str().unwrap()),
            }
        }   
    };
}

