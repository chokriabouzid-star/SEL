//! SEL Engine CLI - واجهة سطر الأوامر

use clap::{Parser, Subcommand};
use std::fs;
use sel_engine::{SEL, Mission, Result};

#[derive(Parser)]
#[command(name = "sel-engine")]
#[command(about = "Sovereign Execution Layer - Pure Execution Engine")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// تنفيذ مهمة
    Execute {
        /// ملف المهمة (JSON)
        #[arg(short, long)]
        mission: String,
        
        /// ملف إخراج الحقائق
        #[arg(short, long, default_value = "facts.jsonl")]
        output: Option<String>,
        
        /// عدم التحقق من المهمة (للتطوير فقط)
        #[arg(long, default_value = "false")]
        no_validate: bool,
    },
    
    /// التحقق من صحة المهمة فقط
    Validate {
        /// ملف المهمة (JSON)
        #[arg(short, long)]
        mission: String,
    },
    
    /// عرض معلومات عن مهمة
    Inspect {
        /// ملف المهمة (JSON)
        #[arg(short, long)]
        mission: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Execute { mission, output, no_validate } => {
            execute_mission(&mission, output.as_deref(), no_validate)
        }
        
        Commands::Validate { mission } => {
            validate_mission_cli(&mission)
        }
        
        Commands::Inspect { mission } => {
            inspect_mission(&mission)
        }
    }
}

/// تنفيذ مهمة
fn execute_mission(mission_file: &str, output: Option<&str>, no_validate: bool) -> Result<()> {
    println!("🚀 SEL Engine - تنفيذ مهمة");
    println!("📄 الملف: {}", mission_file);
    
    // قراءة المهمة
    let content = fs::read_to_string(mission_file)?;
    
    let mission: Mission = serde_json::from_str(&content)?;
    
    println!("🎯 المهمة: {} (v{})", mission.id, mission.version);
    println!("🛠️  عدد الإجراءات: {}", mission.execution.actions.len());
    
    // التحقق من المهمة (ما لم يُطلب التخطي)
    if !no_validate {
        println!("🔍 التحقق من صحة المهمة...");
        match sel_engine::validate_mission(&mission) {
            Ok(_) => println!("✅ المهمة صالحة"),
            Err(e) => {
                eprintln!("❌ خطأ في التحقق:");
                eprintln!("{}", e);
                eprintln!("\nℹ️  للتخطي، استخدم: --no-validate");
                std::process::exit(1);
            }
        }
    }
    
    // تنفيذ المهمة
    println!("⚡ تنفيذ المهمة...");
    let mut sel = SEL::new(&mission.id)?;
    
    match sel.execute(mission) {
        Ok(_) => {
            let facts_path = sel.facts_path();
            println!("✅ تم تنفيذ المهمة بنجاح");
            println!("📊 الحقائق: {}", facts_path.display());
            
            // إذا طلب المستخدم ملف إخراج معين
            if let Some(output_path) = output {
                if output_path != facts_path.to_string_lossy() {
                    fs::copy(facts_path, output_path)?;
                    println!("📁 نسخ الحقائق إلى: {}", output_path);
                }
            }
            
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ خطأ في التنفيذ: {}", e);
            std::process::exit(1);
        }
    }
}

/// التحقق من مهمة (CLI)
fn validate_mission_cli(mission_file: &str) -> Result<()> {
    println!("🔍 SEL Engine - التحقق من المهمة");
    
    let content = fs::read_to_string(mission_file)?;
    
    let mission: Mission = serde_json::from_str(&content)?;
    
    println!("📄 الملف: {}", mission_file);
    println!("🎯 المهمة: {} (v{})", mission.id, mission.version);
    println!("🛠️  عدد الإجراءات: {}", mission.execution.actions.len());
    
    match sel_engine::validate_mission(&mission) {
        Ok(_) => {
            println!("✅ المهمة صالحة");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ المهمة غير صالحة:");
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

/// فحص مهمة
fn inspect_mission(mission_file: &str) -> Result<()> {
    println!("🔎 SEL Engine - فحص المهمة");
    
    let content = fs::read_to_string(mission_file)?;
    
    let mission: Mission = serde_json::from_str(&content)?;
    
    println!("📄 الملف: {}", mission_file);
    println!("🎯 المهمة: {}", mission.id);
    println!("📦 الإصدار: {}", mission.version);
    println!("🛠️  عدد الإجراءات: {}", mission.execution.actions.len());
    
    println!("\n📋 معلومات الإجراءات:");
    for (i, action) in mission.execution.actions.iter().enumerate() {
        println!("\n  🔹 الإجراء #{}:", i + 1);
        println!("    ID: {}", action.id);
        println!("    النوع: {}", action.action_type);
        println!("    الأمر: {}", action.command);
        
        if let Some(ref args) = action.args {
            println!("    الوسيطات: {}", args.join(" "));
        }
        
        println!("    مسار العمل: {}", action.working_directory);
        
        if let Some(ref env) = action.environment {
            println!("    متغيرات البيئة: {}", env.len());
            for (key, value) in env.iter().take(3) {
                println!("      - {}={}", key, value);
            }
            if env.len() > 3 {
                println!("      ... و {} أخرى", env.len() - 3);
            }
        }
    }
    
    println!("\n📊 حجم المهمة: {} bytes", content.len());
    println!("📁 المسار: {}", mission_file);
    
    Ok(())
}
