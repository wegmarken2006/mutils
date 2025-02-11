
/// Mutable Vec<_> init
///
/// Use: vi!(var_name, type);
#[macro_export]
macro_rules! vi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<$e> = vec![];
    };
}

/// Mutable Vec<Vec<_>> init.
///
/// Use: vvi!(var_name, type);
#[macro_export]
macro_rules! vvi {
    ($i:tt, $e:ty) => {
        let mut $i: Vec<Vec<$e>> = vec![];
    };
}

/// HashMap<_,_> init.
///
/// Use: hmi!(var_name, key_type, val_type);
///
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
///
/// Use: hms!(var_name, key, val);
///
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
///
/// Use: hmt!(key_type, val_type)
#[macro_export]
macro_rules! hmt {
    ($a:ty, $b:ty) => {
        std::collections::HashMap<$a, $b>
    };
}

/// Vec<Vec<_>>  type
///
/// Use: vvt!(type)
#[macro_export]
macro_rules! vvt {
    ($e:ty) => {
        Vec<Vec<$e>>
    };
}

/// Vec<_> type
///
/// Use: vt!(type)
#[macro_export]
macro_rules! vt {
    ($e:ty) => {
        Vec<$e>
    };
}

/// Get char from String
///
/// Use: chg!(string_name, position, char_var_name);
///
/// Example:
/// ```
///     let mut st = "abc".to_string();
///     chg!(st, 1, c1);
///     println!("{}", c1);
/// ```
#[macro_export]
macro_rules! chg {
    ($i:tt, $n:tt, $out:tt) => {
        let $out = $i.chars().nth($n).unwrap();
    };
}

/// Set char in String
///
/// Use: chs!(string_name, position, char_val_as_str);
///
/// Example:
/// ```
///     let mut st = "abc".to_string();
///     chs!(st, 2, "d");
///     println!("{}", st);
/// ```
macro_rules! chs {
    ($i:tt, $n:tt, $r:tt) => {
        $i.replace_range($n..$n + 1, $r);
    };
}

/// For enumerate.
///
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
///
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
///
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

/// CSV file with headers read.
///
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
            .has_headers(true)
            .delimiter($del)
            .flexible(true)
            .from_path($fpath)
            .expect("Error reading CSV");
        for result in rdr.records() {
            let $rec = result.expect("No record");
            $block
        }
    };
}

/// CSV file with no headers read.
///
/// Example:
/// ```
///    use csv;
/// 
///    csvread_nh!("example.csv", b',', record, {
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
macro_rules! csvread_nh {
    ($fpath:tt, $del:tt, $rec:tt, $block:expr) => {        
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter($del)
            .flexible(true)
            .from_path($fpath)
            .expect("Error reading CSV");
        for result in rdr.records() {
            let $rec = result.expect("No record");
            $block
        }
    };
}

/// Unbounded channel init.
///
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
///
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
///
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

#[macro_export]
/// A thread shareable variable.
///
/// Example:
/// ```
///     use crossbeam_channel;
///     uchani!(tx1, rx1, i32);
///     uchani!(tx2, rx2, i32);
///
///     let count = 0;
///     shareable!(count, shareable);
///
///     sharec!(shareable, c1);
///     std::thread::spawn(move || {
///         loop {
///             chan_rx!(rx1, n, {
///                 println!("Thread 1 {}", n);
///             });
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             shareg!(c1, cc);
///             cc = cc + 1;
///             shares!(cc, c1);
///             chan_tx!(tx2, cc);
///         }
///     });
///
///     sharec!(shareable, c2);
///     std::thread::spawn(move || {
///         loop {
///             chan_rx!(rx2, n, {
///                 println!("Thread 2 {}", n);
///             });
///             std::thread::sleep(std::time::Duration::from_secs(2));
///             shareg!(c2, cc);
///             cc = cc + 10;
///             shares!(cc, c2);
///             chan_tx!(tx1, cc);
///         }
///     });
/// ```
macro_rules! shareable {
    ($i:tt, $out:tt) => {
        let sh = std::sync::Arc::new(std::sync::Mutex::new($i));
        let $out = sh.clone();
    };
}

/// Clone a shareable variable.
///
/// Use: sharec!(shareable_var, cloned_var);
#[macro_export]
macro_rules! sharec {
    ($i:tt, $out:tt) => {
        let $out = $i.clone();
    };
}

/// Get a mutable shareable variable.
///
/// Use: shareg!(cloned_var, modifiable_var);
#[macro_export]
macro_rules! shareg {
    ($i:tt, $out:tt) => {
        let mut $out =  *$i.lock().unwrap();
    };
}

/// Set a shareable variable.
///
/// Use: shares!(modified_var, cloned_var);
#[macro_export]
macro_rules! shares {
    ($i:tt, $out:tt) => {
        *$out.lock().unwrap() = $i;
    };
}

/// Mutable shareable variable type.
///
/// Use: sharet!(type)
#[macro_export]
macro_rules! sharet {
    ($i:ty) => {
        std::sync::Arc<std::sync::Mutex<$i>>
    };
}

/// Get a mutable shareable variable with clone for not Copy-able vars
///
/// Use: sharegc!(cloned_var, modifiable_var);
/// 
/// Example:
/// 
/// ```
///     let str1 = "Hello".to_string();
///     shareable!(str1, sstr1);
///     sharegc!(sstr1, str2);
///     str2 = "Hello world".to_string();
///     shares!(str2, sstr1);
/// ```
#[macro_export]
macro_rules! sharegc {
    ($i:tt, $out:tt) => {
        let mut $out =  $i.lock().unwrap().clone();
    };
}

/// Type "Function of 1 argument"
///
/// Use: funt!(arg_type)
#[macro_export]
macro_rules! funt {
    ($e:ty) => {
        impl Fn($e) + std::marker::Send + 'static
    };
}

/// Sleep, in seconds
///
/// Use: ssleep!(seconds)
#[macro_export]
macro_rules! ssleep {
    ($e:tt) => {
        std::thread::sleep(std::time::Duration::from_secs($e));
    };
}


