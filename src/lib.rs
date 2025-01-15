
/// Mutable Vec<_> init
#[macro_export]
macro_rules! vi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<$e> = vec![];
    };
}

/// Mutable Vec<Vec<_>> init.
#[macro_export]
macro_rules! vvi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<Vec<$e>> = vec![];
    };
}

/// HashMap<_,_> init.
/// Example:
/// ```
///    hmi!(map1, &str, i32);
/// ```
#[macro_export]
macro_rules! hmi {
    ($i:tt, $a:ty, $b:ty) => {
        let mut $i: std::collections::HashMap<$a, $b> = std::collections::HashMap::new();
    };
}

/// HasMap set.
/// Example:
/// ```
///    hmi!(map1, &str, i32);
///    hms!(hm, "key1", 10);
/// ```
#[macro_export]
macro_rules! hms {
    ($i:tt, $key:tt, $val:tt) => {
        let hh = $i.entry($key).or_default();
        *hh = $val;
    };
}

/// HashMap<_,_> type
#[macro_export]
macro_rules! hmt {
    ($a:ty, $b:ty) => {
        std::collections::HashMap<$a, $b>
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

/// For enumerate.
/// Example:
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

/// Read lines from file.
/// Example:
/// ```
///    use std::io::BufRead;
///
///    flines!("Cargo.toml", lines);
///    println!("{:?}", lines);
/// ```
#[macro_export]
macro_rules! flines {
    ($fn:tt, $lines:tt) => {
        let f = std::fs::File::open($fn).expect(&format!("File {} not found", $fn));
        let file = std::io::BufReader::new(&f);
        let $lines = file.lines();
    };
}

/// File walk.
/// Example:
/// ```
///    fwalk!("./", "toml", file_path, 
///        println!("{}", file_path);
///    });
/// ```
#[macro_export]
macro_rules! fwalk {
    ($sdir:tt, $ext:tt, $fpath:tt, $block:expr) => {
        fn all_files(sdir: &str, ext: &str) {
            let root_path = std::path::Path::new(sdir);
            if root_path.is_dir() {
                match std::fs::read_dir(root_path) {
                    Ok(dirs) => {
                        for entry in dirs {
                            let entry = entry.unwrap();
                            let path = entry.path();
                            if path.is_dir() {
                                all_files(path.to_str().unwrap(), ext);
                            } else if path.extension().unwrap_or_default() == $ext || ext == "*" {
                                let $fpath = format!("{}", path.display());
                                $block
                            }
                        }
                    }
                    Err(_) => println!("No permissions for: {}", root_path.to_str().unwrap()),
                }
            }   
        }
        all_files($sdir, $ext);
    };
}

/// CSV file read.
/// Example:
/// ```
///    use csv;
/// 
///    csvread!("example.csv", b',', record, {
///         let first: i32 = match record[0].parse() {
///             Ok(st) => st,
///         Err(_) => 0,
///         };
///         let second: String = match record[1].parse() {
///             Ok(st) => st,
///             Err(_) => String::from(""),
///         };
///    
///         println!("First: {}, Second: {}", first, second);    
///    });
/// ```
#[macro_export]
macro_rules! csvread {
    ($fpath:tt, $del:tt, $rec:tt, $block:expr) => {        
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter($del)
            .from_path($fpath)
            .expect("Error reading CSV");
        for result in rdr.records() {
            let $rec = result.expect("No record");
            $block
        }
    };
}

/// Unbounded channel init.
/// Example:
/// ```
///    use crossbeam_channel;
/// 
///    uchani!(tx1, rx1, i32);
/// ```
#[macro_export]
macro_rules! uchani {
    ($ser:tt, $rer:tt, $tp:ty) => {
        let ($ser, $rer): (
            crossbeam_channel::Sender<$tp>,
            crossbeam_channel::Receiver<$tp>,
        ) = crossbeam_channel::unbounded();
    };
}

/// Channel receive.
/// Example:
/// ```
///     use crossbeam_channel;
/// 
///     uchani!(tx1, rx1, i32);
///     uchani!(tx2, rx2, &str);
///     std::thread::spawn(move || {
///         loop {
///             chan_rx!(rx1, n, {
///                println!("Thread 1 {}", n);
///             });
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             chan_tx!(tx2, "From thread 1");
///         }
///     });
///
///     std::thread::spawn(move || {
///         let mut count = 100;
///         loop {
///             chan_rx!(rx2, n, {
///                 println!("Thread 2 {}", n);
///             });
///
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             chan_tx!(tx1, count);
///             count = count + 1;
///         }
///     });
/// ```
#[macro_export]
macro_rules! chan_rx {
    ($rx:tt, $out:tt, $block:expr) => {
        match $rx.try_recv() {
            Ok($out) => {$block},
            Err(_) => (),
        }
    };
}

/// Channel send
/// Example:
/// ```
///     use crossbeam_channel;
/// 
///     uchani!(tx1, rx1, i32);
///     uchani!(tx2, rx2, &str);
///     std::thread::spawn(move || {
///         loop {
///             chan_rx!(rx1, n, {
///                println!("Thread 1 {}", n);
///             });
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             chan_tx!(tx2, "From thread 1");
///         }
///     });
///
///     std::thread::spawn(move || {
///         let mut count = 100;
///         loop {
///             chan_rx!(rx2, n, {
///                 println!("Thread 2 {}", n);
///             });
///
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             chan_tx!(tx1, count);
///             count = count + 1;
///         }
///     });
/// ```
#[macro_export]
macro_rules! chan_tx {
    ($tx:tt, $out:tt) => {
        $tx.try_send($out).unwrap();
    };
}
