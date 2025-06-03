// Rust CLI Tool
// Author: Gabriel Demetrios Lafis

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "rust-cli-tool")]
#[command(about = "Professional CLI tool by Gabriel Demetrios Lafis")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze data from a file
    Analyze {
        /// Input file path
        #[arg(short, long)]
        file: String,
    },
    /// Generate a report
    Report {
        /// Report type
        #[arg(short, long, default_value = "summary")]
        type_: String,
    },
    /// Show system information
    Info,
}

#[derive(Serialize, Deserialize)]
struct DataPoint {
    id: u32,
    value: f64,
    category: String,
}

#[derive(Serialize, Deserialize)]
struct Report {
    title: String,
    author: String,
    timestamp: String,
    data: Vec<DataPoint>,
    summary: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    println!("🦀 Rust CLI Tool");
    println!("👨‍💻 Created by Gabriel Demetrios Lafis\n");
    
    match cli.command {
        Commands::Analyze { file } => {
            analyze_file(&file).await;
        }
        Commands::Report { type_ } => {
            generate_report(&type_).await;
        }
        Commands::Info => {
            show_info().await;
        }
    }
}

async fn analyze_file(file_path: &str) {
    println!("📊 Analyzing file: {}", file_path);
    
    // Simulate data analysis
    let sample_data = vec![
        DataPoint { id: 1, value: 100.5, category: "A".to_string() },
        DataPoint { id: 2, value: 200.3, category: "B".to_string() },
        DataPoint { id: 3, value: 150.7, category: "A".to_string() },
    ];
    
    let mut summary = HashMap::new();
    for point in &sample_data {
        *summary.entry(point.category.clone()).or_insert(0.0) += point.value;
    }
    
    println!("✅ Analysis complete!");
    println!("📈 Summary:");
    for (category, total) in summary {
        println!("  {}: {:.2}", category, total);
    }
}

async fn generate_report(report_type: &str) {
    println!("📄 Generating {} report...", report_type);
    
    let report = Report {
        title: format!("{} Report", report_type.to_uppercase()),
        author: "Gabriel Demetrios Lafis".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        data: vec![
            DataPoint { id: 1, value: 100.0, category: "Performance".to_string() },
            DataPoint { id: 2, value: 95.5, category: "Quality".to_string() },
        ],
        summary: [
            ("total_items".to_string(), 2.0),
            ("avg_value".to_string(), 97.75),
        ].iter().cloned().collect(),
    };
    
    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            println!("✅ Report generated successfully!");
            println!("{}", json);
        }
        Err(e) => println!("❌ Error generating report: {}", e),
    }
}

async fn show_info() {
    println!("ℹ️  System Information");
    println!("🦀 Rust CLI Tool v1.0.0");
    println!("👨‍💻 Author: Gabriel Demetrios Lafis");
    println!("🏗️  Built with Rust and Clap");
    println!("📅 Build Date: {}", env!("CARGO_PKG_VERSION"));
    
    // System info
    println!("\n💻 Runtime Information:");
    println!("  OS: {}", std::env::consts::OS);
    println!("  Architecture: {}", std::env::consts::ARCH);
    println!("  Rust Version: {}", env!("CARGO_PKG_RUST_VERSION"));
}
