use std::time::Duration;
use std::thread;
use std::process::Command;

fn main() {
    for i in 1..100 {        
        println!("{}", i);
        let i_str = i.to_string();
        if i % 3 == 0 || contains(i_str){
            println!("박수");
            say("박수".to_string());
        } else {
            say(i.to_string());
        }
        thread::sleep(Duration::from_millis(100))
    }
}

fn contains(a: String) -> bool {
    for i in &["3", "6", "9"] {
        if a.contains(i){
            return true
        }
    }
    return false
}

fn say(command: String) {
    Command::new("say").arg("-v").arg("Yuna").arg(command).output().expect("failed to execute process");
}
