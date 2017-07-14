
//this is just a tool.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Write;
use std::io::Read;

fn system2(cmd:&str) {
    let mut process = Command::new("cmd.exe").stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn().expect("command run failed");

    {
        let stdin = process.stdin.as_mut().expect("get stdin failed");
        //stdin.writeln!(, "c:").expect("failed");
        stdin.write_all(b"c:\n").expect("failed to write");
        stdin.write_all(b"cd 'C:\\Program Files\\WinRAR'").expect("failed to write");
        stdin.write_all(cmd.as_bytes()).expect("failed to write cmd");
    }

    //let output = process.stdout;
    let mut str = String::new();
    let ok = process.stdout.unwrap().read_to_string(&mut str);
    //println!("output: {:?}", String::from_utf8_lossy(&output.stdout));
    println!("{:?}", str);
}

/*
 * from the official document, there is a little difference
 * between OSs
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
 */
fn system(source:&PathBuf, target:&PathBuf) {
    let mut output =
        Command::new("cmd")
        .env("path", "C:/Program Files/WinRAR/")
        .arg("/C")
        .arg( "WinRAR.exe")
        .arg("a")
        .arg("-k")
        .arg("-r")
        .arg("-s")
        .arg("-ibck")
        .arg(target)
        .arg(source)
        .output()
        .expect("command run failed");
    /*
    let mut output2 = Command::new("cmd")
        .args(&["/C", "ping baidu.com"])
        .output().expect("command run failed");
        */
    //process
    //println!("22222222222222222");
    // It's streaming here

    if output.status.success() {
        println!("ok output:  {:?}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("err output:  {:?}", String::from_utf8_lossy(&output.stderr));
    }
   
}

fn backup( path:&PathBuf) {
    let t:String= format!("{}.rar", path.to_string_lossy());

    println!("start to compress: {:?}", t);
    let target = PathBuf::from(t);
    //let target = path.extend(".rar");
    //let source = path.as_u8_slice();
    //let target = t.as_bytes();
    system(path , &target);
    //println!("{:?}", cmd);
    fs::remove_dir_all(path).unwrap();
}

/*
 * list all directorys under specific dir
 * compress them one by one 
 * then delete it.
 */
fn list_files(path:&str) {
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let f = file.unwrap();
        let filetype = f.file_type().unwrap();
        //let name = f.file_name();
        //this is a full path
        let path = f.path();

        if filetype.is_dir() {
            backup(&path);
        }
        //println!("{:?},{:?},{:?}, ", filetype, name, path);
    }
}

fn main() {
    list_files("e:/DealDetails/");
    println!("done");
}
