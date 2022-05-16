use std::io::{BufRead, BufReader};
use std::process::Command;
use std::fs;

// #![cfg_attr(
#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {

  read_base_list();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command,btn_cmd,close_window,open1c])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");

  Command::new("cmd")
  .args(["/C", "start calc"])
  .output()
  .expect("failed to execute process");
}

#[tauri::command]
fn btn_cmd() {
  println!("I was invoked from JS!");

  Command::new("cmd")
  .args(["/C", "start cmd"])
  .output()
  .expect("failed to execute process");
}

#[tauri::command]
fn close_window(name:String){
  println!("I was invoked from JS!");
  println!("I was invoked from JS {}", name);
}

#[tauri::command]
fn open1c(){
  // let path:&str = "C:\\dev\\base\\Новая папка";
  println!("I was invoked from JS!");
  Command::new("cmd")
  .args(["/C", "C:/Program Files/1cv8/common/1cestart.exe","ENTERPRISE","/DisableStartupMessages","/F","C:/dev/base/new"])
  .output()
  .expect("failed to execute process");
}

//working with file


fn read_base_list() {
  let base_list:&str = "C:/Users/Front/AppData/Roaming/1C/1CEStart/ibases.v8i";


  let file = fs::File::open(base_list).expect("cannot open file");
  let file = BufReader::new(file);
  for line in file.lines() {
      // print!("\n{:?}", line.unwrap().find('='));
      print!("\n{:?}", line.unwrap());



      if (line.unwrap().as_ref().contains("Connect=File")) {
        // print!("\n contain {:?}",line.unwrap());
      }
      // print!("\n{:?}",line);
      // line.find("=")
      // print_type_of(line);
      // print!("{}",line.split("="));
  }

  // let contents = fs::read_to_string(base_list)
  // .expect("Something went wrong reading the file");

  // println!("With text:\n{}", contents);
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}


