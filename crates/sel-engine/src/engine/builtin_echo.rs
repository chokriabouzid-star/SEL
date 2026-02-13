//! SEL Built-in Echo - لا يعتمد على أي binaries خارجية
//! SOVEREIGN: نكتب stdout مباشرة بدون أي أوامر نظام
//! DETERMINISTIC: لا نسجل مسارات مطلقة في المخرجات

use std::io::Write;

pub fn builtin_echo(args: &[String]) -> (i32, String, String) {
    let mut stdout = Vec::new();
    
    if args.is_empty() {
        writeln!(stdout).unwrap();
    } else {
        let output = args.join(" ");
        writeln!(stdout, "{}", output).unwrap();
    }
    
    (
        0,
        String::from_utf8_lossy(&stdout).to_string(),
        String::new(),
    )
}

pub fn builtin_pwd() -> (i32, String, String) {
    // DETERMINISTIC: نعيد "." دائماً - ثابت في كل مرة
    (
        0,
        ".\n".to_string(),
        String::new(),
    )
}
