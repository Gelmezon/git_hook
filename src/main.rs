
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty(){
        eprintln!("No file paths provided.");
        std::process::exit(1);
    }

    //使用alibaba-java-format工具格式化Java文件
    let formatter_jar = "google-java-format-1.27.0-all-deps.jar";
    if !Path::new(formatter_jar).exists(){
           eprintln!("Formatter jar file '{}' does not exist.", formatter_jar);
           std::process::exit(1); 
    }

        // 遍历每个文件并格式化
    for file in &args {
        if file.ends_with(".java") {
            if let Err(e) = format_java_file(file, formatter_jar) {
                eprintln!("Error formatting {}: {}", file, e);
            }
        }
    }


}


fn format_java_file(file:&str,formatter_jar:&str) -> std::io::Result<()> {
   // 读取文件内容
   let content = match fs::read_to_string(file){
         Ok(content) => content,
         Err(e) => {
              eprintln!("Error reading file '{}': {}", file, e);
              return Err(e);
         }
   };

    let output = Command::new("java")
        .arg("-jar")
        .arg(formatter_jar)
        .arg("-")
        .arg("--aosp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            // 写入文件内容到 stdin
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(content.as_bytes())?;
            }
            child.wait_with_output()
        });

    match output {
        Ok(output) if output.status.success() => {
            if let Err(e) = fs::write(file, &output.stdout) {
                eprintln!("Failed to write formatted content to {}: {}", file, e);
                return Err(e);
            } else {
                println!("Formatted: {}", file);
            }
            Ok(())
        }
        Ok(output) => {
            eprintln!("Formatting failed for {}: {}", file, String::from_utf8_lossy(&output.stderr));
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Formatting failed"))
        }
        Err(e) => {
            eprintln!("Failed to execute formatter for {}: {}", file, e);
            Err(e)
        }
    }
}