use rust_search::{SearchBuilder, FilterExt};
use std::sync::Mutex;
use lazy_static::lazy_static;
use rfd::FileDialog;
use std::process::Command;
use std::cell::RefCell;
use std::path::PathBuf;

lazy_static! {
        static ref SINGLETON_SEARCH: Mutex<SukerSearch> = Mutex::new(SukerSearch::default());
}

struct SukerSearch {
        search_result: RefCell<Vec<String>>,
        search_done: bool,
}
impl Default for SukerSearch {
    fn default() -> Self {
        SukerSearch {
            search_result: RefCell::new(Vec::new()),
            search_done: false,
        }
    }
}

fn init() {
        //SINGLETON_SEARCH clear
        SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().clear();
        SINGLETON_SEARCH.lock().unwrap().search_done = false;
        println!("init");
        println!("{}", &SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().len());
}

#[tauri::command]
pub fn rust_open_dir_dialog() -> String {
    //open dialog and directory path return
    let files = FileDialog::new()    
    .set_directory("/")
    .pick_folder();

    match files {
        Some(file) => {
            return format!("{}", file.display().to_string());
        },
        None => "No file selected".to_string(),
    }
}

#[tauri::command]
pub fn rust_open_dir_dialog_standalone(path: &str) {
    let path = PathBuf::from(path);
    let real_dir_path;
    if path.is_dir() {
        println!("is dir");
        real_dir_path = path.to_str().unwrap().to_string();
    } else {
        println!("is not dir");
        real_dir_path = path.parent().unwrap().to_str().unwrap().to_string();
    }
    println!("{}", real_dir_path);
    Command::new("explorer")
        .arg(real_dir_path)
        .spawn()
        .expect("failed to execute process");
}

#[tauri::command]
pub fn do_search_file(loc_start: &str, in_search: &str,
        limit: usize, ext: &str, depth: usize, hidden: bool, strict: bool)
{
    init();
    println!("do_search_file\n");

    //if inSearch is '*' then change empty string
    let in_search = if in_search == "*" { "" } else { in_search };

    let loc_more = vec![""];    
    let mut builder = SearchBuilder::default()
        .location(loc_start) // 검색 시작 위치
        .search_input(in_search) // 찾을 파일 or DIR
        .more_locations(loc_more) // 위치 추가
        .limit(limit) // 갯수
        .ext(ext) // 확장자
        .depth(depth); // 깊이

    if strict {
        builder = builder.strict(); // 정확도
    }
    if hidden {
        builder = builder.hidden(); // hidden file
    }

    let collected: Vec<String> = builder.build().collect();

    SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().extend(collected);
    SINGLETON_SEARCH.lock().unwrap().search_done = true;

    //debug print    
    println!("locStart = {}\n", loc_start);
    println!("inSearch = {}\n", in_search);
    println!("limit = {}\n", limit);
    println!("ext = {}\n", ext);
    println!("depth = {}\n", depth);
    println!("hidden = {},  strict = {}\n", hidden, strict);
}

//find folder
#[tauri::command]
pub fn do_search_dir(loc_start: &str, in_search: &str,
                                            limit: usize, _ext: &str, depth: usize, hidden: bool, strict: bool)
{
    init();
    println!("do_search_dir\n");
    let loc_more = vec![""];

    //if in_search is '*' then change empty string
    let in_search = if in_search == "*" { "" } else { in_search };
    let mut builder = SearchBuilder::default()
        .location(loc_start) // 검색 시작 위치
        .search_input(in_search) // 찾을 파일 or DIR
        .more_locations(loc_more) // 위치 추가
        .limit(limit) // 갯수        
        .depth(depth) // 깊이
        .custom_filter(|in_search| !in_search.metadata().unwrap().is_file()); // 디렉토리만 찾기

    if strict {
        builder = builder.strict(); // 정확도
    }
    if hidden {
        builder = builder.hidden(); // hidden file
    }

    let collected: Vec<String> = builder.build().collect();

    SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().extend(collected);
    SINGLETON_SEARCH.lock().unwrap().search_done = true;
}

#[tauri::command]
pub fn rust_check_search_done() -> bool {
    SINGLETON_SEARCH.lock().unwrap().search_done
}

#[tauri::command]
pub fn rust_make_vec_result() -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    for path in SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().iter() {
        println!("{}", path);
        ret.push(path.to_string());
    }

    println!("result count = {}", SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().len());
    SINGLETON_SEARCH.lock().unwrap().search_result.borrow_mut().clear();
    ret
}