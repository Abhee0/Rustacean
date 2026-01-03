# 30-Day Rust Learning Project: Building a Personal Data Dashboard

## 🎯 Project Overview

This is a **complete, hands-on Rust project** that grows with your skills over 30 days. You'll build a **Personal Data Dashboard** - a real-world application that teaches you Rust fundamentals through incremental development.

### What You'll Build

By the end of 30 days, you'll have:
- ✅ A command-line data dashboard
- ✅ File-based data persistence
- ✅ Multi-threaded data processor
- ✅ Web API server
- ✅ Real-time statistics and analytics
- ✅ Export functionality (JSON, CSV)
- ✅ Error handling and validation
- ✅ Comprehensive testing

### Project Architecture (Grows Over Time)

```
Week 1: CLI Data Entry System
Week 2: Data Storage & Collections
Week 3: Memory Management & Processing
Week 4: Concurrency & Real-world Features
```

---

## 📅 COMPLETE 30-DAY TIMELINE

### WEEK 1: FUNDAMENTALS (Days 1-6)
**Goal**: Build a working CLI data entry system

---

## 🗓️ DAY 1: Project Setup & Hello Dashboard

**Duration**: 3-4 hours  
**Skills**: Rust basics, Cargo, project structure

### Learning Objectives
- Set up Rust development environment
- Create project structure
- Understand Cargo and dependencies
- Build first runnable dashboard

### Tasks

#### 1.1 Create Project
```bash
cargo new personal_dashboard
cd personal_dashboard
```

#### 1.2 Create Project Structure
```
personal_dashboard/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models.rs
│   ├── cli.rs
│   └── storage.rs
├── data/
│   └── .gitkeep
└── tests/
    └── integration_tests.rs
```

#### 1.3 Update Cargo.toml
```toml
[package]
name = "personal_dashboard"
version = "0.1.0"
edition = "2021"

[dependencies]
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
assert_cmd = "2.0"
```

#### 1.4 Create lib.rs
```rust
// src/lib.rs
pub mod models;
pub mod cli;
pub mod storage;

pub fn app_version() -> &'static str {
    "v0.1.0"
}

pub fn app_name() -> &'static str {
    "Personal Dashboard"
}
```

#### 1.5 Create main.rs
```rust
// src/main.rs
use personal_dashboard::{app_name, app_version};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║       {} {}       ║", app_name(), app_version());
    println!("║    Your Personal Data Hub            ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("Welcome! Starting your dashboard...\n");
    
    show_menu();
}

fn show_menu() {
    println!("Main Menu:");
    println!("1. Add Entry");
    println!("2. View Dashboard");
    println!("3. Export Data");
    println!("4. Exit\n");
    println!("Select an option (1-4):");
}
```

#### 1.6 Create models.rs (Empty Structure)
```rust
// src/models.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub notes: String,
}

impl DataEntry {
    pub fn new(
        id: u32,
        title: String,
        category: String,
        value: f64,
        notes: String,
    ) -> Self {
        DataEntry {
            id,
            title,
            category,
            value,
            timestamp: Utc::now(),
            notes,
        }
    }
}
```

#### 1.7 Create cli.rs (Empty Structure)
```rust
// src/cli.rs
pub fn read_user_input() -> String {
    use std::io::{self, Write};
    
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_option(max: u32) -> Option<u32> {
    let input = read_user_input();
    input.parse::<u32>().ok().filter(|&n| n >= 1 && n <= max)
}
```

#### 1.8 Create storage.rs (Empty Structure)
```rust
// src/storage.rs
use crate::models::DataEntry;

pub struct DataStore {
    entries: Vec<DataEntry>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: DataEntry) {
        self.entries.push(entry);
    }
    
    pub fn get_entries(&self) -> &[DataEntry] {
        &self.entries
    }
}
```

### Testing
```bash
cargo run
# Should show welcome message
```

### Exercises

**Exercise 1.1**: Modify the welcome message
- Change app name and version
- Add your name to welcome

**Exercise 1.2**: Add more menu options
- Add "Statistics"
- Add "Settings"
- Update show_menu()

**Exercise 1.3**: Create unit test
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_name() {
        assert_eq!(app_name(), "Personal Dashboard");
    }
}
```

**Exercise 1.4**: Document your code
- Add doc comments to all public functions
- Run `cargo doc --open`

### Milestone ✅
- ✅ Project structure created
- ✅ Basic CLI running
- ✅ Models defined
- ✅ Understanding Cargo and project layout

---

## 🗓️ DAY 2: User Input & Data Entry

**Duration**: 3-4 hours  
**Skills**: Variables, functions, user input, string handling

### Learning Objectives
- Handle user input properly
- Create interactive data entry
- Build data validation
- Use variables and types correctly

### Tasks

#### 2.1 Enhance main.rs - Interactive Menu
```rust
// src/main.rs (Updated)
use personal_dashboard::{app_name, app_version, models::DataEntry, storage::DataStore, cli};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║       {} {}       ║", app_name(), app_version());
    println!("║    Your Personal Data Hub            ║");
    println!("╚════════════════════════════════════════╝\n");
    
    let mut store = DataStore::new();
    let mut running = true;
    
    while running {
        show_menu();
        
        match cli::get_option(4) {
            Some(1) => add_entry(&mut store),
            Some(2) => view_dashboard(&store),
            Some(3) => println!("Export functionality coming soon!"),
            Some(4) => {
                println!("Goodbye!");
                running = false;
            }
            _ => println!("Invalid option. Please try again.\n"),
        }
    }
}

fn show_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║           MAIN MENU                  ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ 1. Add Entry                         ║");
    println!("║ 2. View Dashboard                    ║");
    println!("║ 3. Export Data                       ║");
    println!("║ 4. Exit                             ║");
    println!("╚════════════════════════════════════════╝\n");
    print!("Select option (1-4): ");
}

fn add_entry(store: &mut DataStore) {
    println!("\n--- Add New Entry ---\n");
    
    // Get title
    print!("Enter title: ");
    let title = cli::read_user_input();
    
    // Get category
    print!("Enter category (Health/Finance/Fitness/Mood/Other): ");
    let category = cli::read_user_input();
    
    // Get value
    print!("Enter value (number): ");
    let value = loop {
        match cli::read_user_input().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => print!("Invalid number. Try again: "),
        }
    };
    
    // Get notes
    print!("Enter notes (optional): ");
    let notes = cli::read_user_input();
    
    // Create and add entry
    let id = store.get_entries().len() as u32 + 1;
    let entry = DataEntry::new(id, title, category, value, notes);
    store.add_entry(entry);
    
    println!("✓ Entry added successfully!\n");
}

fn view_dashboard(store: &DataStore) {
    println!("\n--- Dashboard ---\n");
    
    if store.get_entries().is_empty() {
        println!("No entries yet. Add some data to see your dashboard!\n");
        return;
    }
    
    println!("{:<5} {:<20} {:<15} {:<10}", "ID", "Title", "Category", "Value");
    println!("{:-<50}", "");
    
    for entry in store.get_entries() {
        println!("{:<5} {:<20} {:<15} {:<10.2}", 
                 entry.id, entry.title, entry.category, entry.value);
    }
    println!();
}
```

#### 2.2 Enhance cli.rs
```rust
// src/cli.rs (Updated)
use std::io::{self, Write};

pub fn read_user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_option(max: u32) -> Option<u32> {
    print!("");  // Flush before reading
    let input = read_user_input();
    match input.parse::<u32>() {
        Ok(n) if n >= 1 && n <= max => Some(n),
        _ => None,
    }
}

pub fn get_float() -> Option<f64> {
    read_user_input().parse::<f64>().ok()
}

pub fn confirm(prompt: &str) -> bool {
    println!("{} (y/n): ", prompt);
    let response = read_user_input().to_lowercase();
    response.starts_with('y')
}
```

#### 2.3 Enhance storage.rs
```rust
// src/storage.rs (Updated)
use crate::models::DataEntry;

pub struct DataStore {
    entries: Vec<DataEntry>,
    next_id: u32,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            entries: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_entry(&mut self, mut entry: DataEntry) {
        entry.id = self.next_id;
        self.entries.push(entry);
        self.next_id += 1;
    }
    
    pub fn get_entries(&self) -> &[DataEntry] {
        &self.entries
    }
    
    pub fn total_entries(&self) -> usize {
        self.entries.len()
    }
}
```

### Testing
```bash
cargo run
# Add a few entries
# View dashboard
```

### Exercises

**Exercise 2.1**: Add input validation
```rust
fn validate_category(cat: &str) -> bool {
    matches!(cat, "Health" | "Finance" | "Fitness" | "Mood" | "Other")
}
```

**Exercise 2.2**: Add numeric statistics
```rust
fn calculate_total(store: &DataStore) -> f64 {
    store.get_entries()
        .iter()
        .map(|e| e.value)
        .sum()
}
```

**Exercise 2.3**: Format output nicely
- Add borders with Unicode characters
- Right-align numbers
- Add colors (using `colored` crate)

**Exercise 2.4**: Write tests for input validation
```rust
#[test]
fn test_valid_category() {
    assert!(validate_category("Health"));
    assert!(!validate_category("Invalid"));
}
```

### Milestone ✅
- ✅ Interactive CLI working
- ✅ Data entry functional
- ✅ Basic dashboard display
- ✅ Input handling and validation

---

## 🗓️ DAY 3: Data Types & Collections

**Duration**: 3-4 hours  
**Skills**: Vectors, HashMap, enums, organizing data

### Learning Objectives
- Master Rust data types
- Use enums for categories
- Organize data efficiently
- Implement filtering

### Tasks

#### 3.1 Refactor models.rs with Enum
```rust
// src/models.rs (Updated)
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Health,
    Finance,
    Fitness,
    Mood,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Health => write!(f, "Health"),
            Category::Finance => write!(f, "Finance"),
            Category::Fitness => write!(f, "Fitness"),
            Category::Mood => write!(f, "Mood"),
            Category::Other => write!(f, "Other"),
        }
    }
}

impl Category {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "health" => Some(Category::Health),
            "finance" => Some(Category::Finance),
            "fitness" => Some(Category::Fitness),
            "mood" => Some(Category::Mood),
            "other" => Some(Category::Other),
            _ => None,
        }
    }
    
    pub fn all() -> [Category; 5] {
        [
            Category::Health,
            Category::Finance,
            Category::Fitness,
            Category::Mood,
            Category::Other,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub id: u32,
    pub title: String,
    pub category: Category,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub notes: String,
}

impl DataEntry {
    pub fn new(
        id: u32,
        title: String,
        category: Category,
        value: f64,
        notes: String,
    ) -> Self {
        DataEntry {
            id,
            title,
            category,
            value,
            timestamp: Utc::now(),
            notes,
        }
    }
}
```

#### 3.2 Enhance storage.rs with HashMap Analytics
```rust
// src/storage.rs (Updated)
use crate::models::{DataEntry, Category};
use std::collections::HashMap;

pub struct DataStore {
    entries: Vec<DataEntry>,
    next_id: u32,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            entries: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_entry(&mut self, mut entry: DataEntry) {
        entry.id = self.next_id;
        self.entries.push(entry);
        self.next_id += 1;
    }
    
    pub fn get_entries(&self) -> &[DataEntry] {
        &self.entries
    }
    
    pub fn get_entries_by_category(&self, category: Category) -> Vec<&DataEntry> {
        self.entries
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }
    
    pub fn get_category_stats(&self) -> HashMap<Category, (usize, f64, f64)> {
        let mut stats: HashMap<Category, (usize, f64, f64)> = HashMap::new();
        
        for entry in &self.entries {
            let (count, sum, max) = stats.entry(entry.category).or_insert((0, 0.0, 0.0));
            *count += 1;
            *sum += entry.value;
            *max = max.max(entry.value);
        }
        
        stats
    }
    
    pub fn total_value(&self) -> f64 {
        self.entries.iter().map(|e| e.value).sum()
    }
    
    pub fn average_value(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        self.total_value() / self.entries.len() as f64
    }
    
    pub fn filter_by_value(&self, min: f64, max: f64) -> Vec<&DataEntry> {
        self.entries
            .iter()
            .filter(|e| e.value >= min && e.value <= max)
            .collect()
    }
}
```

#### 3.3 Update main.rs with Analytics
```rust
// In main.rs, update view_dashboard function

fn view_dashboard(store: &DataStore) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║       DASHBOARD SUMMARY              ║");
    println!("╚════════════════════════════════════════╝\n");
    
    if store.get_entries().is_empty() {
        println!("No entries yet. Add some data!\n");
        return;
    }
    
    println!("Total Entries: {}", store.get_entries().len());
    println!("Total Value: {:.2}", store.total_value());
    println!("Average Value: {:.2}\n", store.average_value());
    
    println!("--- By Category ---");
    let stats = store.get_category_stats();
    for category in &crate::models::Category::all() {
        if let Some((count, sum, max)) = stats.get(category) {
            println!("{}: {} entries | Sum: {:.2} | Max: {:.2}",
                     category, count, sum, max);
        }
    }
    
    println!("\n--- Recent Entries ---");
    println!("{:<5} {:<20} {:<15} {:<10}", "ID", "Title", "Category", "Value");
    println!("{:-<50}", "");
    
    for entry in store.get_entries().iter().rev().take(10) {
        println!("{:<5} {:<20} {:<15} {:<10.2}",
                 entry.id, entry.title, entry.category, entry.value);
    }
    println!();
}
```

### Testing
```bash
cargo run
# Add entries with different categories
# View dashboard to see statistics
```

### Exercises

**Exercise 3.1**: Create filtering function
```rust
fn filter_entries(store: &DataStore, category: Category) -> Vec<&DataEntry> {
    store.get_entries_by_category(category)
}
```

**Exercise 3.2**: Add median calculation
```rust
fn calculate_median(store: &DataStore) -> f64 {
    // Sort and find middle value
}
```

**Exercise 3.3**: Find entry with highest value
```rust
fn find_max_entry(store: &DataStore) -> Option<&DataEntry> {
    store.get_entries().iter().max_by(|a, b| {
        a.value.partial_cmp(&b.value).unwrap()
    })
}
```

**Exercise 3.4**: Write comprehensive tests
```rust
#[test]
fn test_category_conversion() {
    assert_eq!(Category::from_string("health"), Some(Category::Health));
}

#[test]
fn test_total_value() {
    let mut store = DataStore::new();
    store.add_entry(DataEntry::new(0, "Test".to_string(), 
                                   Category::Other, 10.0, "".to_string()));
    assert_eq!(store.total_value(), 10.0);
}
```

### Milestone ✅
- ✅ Enums implemented for categories
- ✅ HashMap for analytics
- ✅ Vector operations and filtering
- ✅ Statistics calculations

---

## 🗓️ DAY 4: Functions & Control Flow

**Duration**: 3-4 hours  
**Skills**: Functions, error handling, pattern matching

### Learning Objectives
- Create modular functions
- Handle errors with Result
- Use pattern matching
- Organize code better

### Tasks

#### 4.1 Create analysis.rs module
```rust
// src/analysis.rs
use crate::models::{DataEntry, Category};
use crate::storage::DataStore;
use std::collections::HashMap;

pub fn calculate_trend(entries: &[&DataEntry], category: Category) -> Option<f64> {
    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| e.category == category)
        .collect();
    
    if filtered.len() < 2 {
        return None;
    }
    
    let first_half: f64 = filtered[..filtered.len() / 2]
        .iter()
        .map(|e| e.value)
        .sum();
    let second_half: f64 = filtered[filtered.len() / 2..]
        .iter()
        .map(|e| e.value)
        .sum();
    
    Some(second_half - first_half)
}

pub fn categorize_by_range(store: &DataStore) -> HashMap<String, Vec<&DataEntry>> {
    let mut ranges = HashMap::new();
    
    for entry in store.get_entries() {
        let range = match entry.value {
            v if v < 0.0 => "Negative".to_string(),
            v if v < 50.0 => "Low (0-50)".to_string(),
            v if v < 100.0 => "Medium (50-100)".to_string(),
            _ => "High (100+)".to_string(),
        };
        
        ranges.entry(range).or_insert_with(Vec::new).push(entry);
    }
    
    ranges
}

pub fn get_category_summary(store: &DataStore) -> HashMap<Category, CategorySummary> {
    let mut summary: HashMap<Category, CategorySummary> = HashMap::new();
    
    for entry in store.get_entries() {
        let cat_sum = summary.entry(entry.category).or_insert_with(|| {
            CategorySummary::new(entry.category)
        });
        
        cat_sum.add_entry(entry);
    }
    
    summary
}

#[derive(Debug, Clone)]
pub struct CategorySummary {
    pub category: Category,
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub average: f64,
}

impl CategorySummary {
    pub fn new(category: Category) -> Self {
        CategorySummary {
            category,
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            average: 0.0,
        }
    }
    
    pub fn add_entry(&mut self, entry: &DataEntry) {
        self.count += 1;
        self.sum += entry.value;
        self.min = self.min.min(entry.value);
        self.max = self.max.max(entry.value);
        self.average = self.sum / self.count as f64;
    }
}
```

#### 4.2 Update lib.rs
```rust
// src/lib.rs (Updated)
pub mod models;
pub mod cli;
pub mod storage;
pub mod analysis;

pub use analysis::{CategorySummary, calculate_trend, get_category_summary};
```

#### 4.3 Add error handling
```rust
// src/storage.rs - Add error handling

#[derive(Debug)]
pub enum StorageError {
    InvalidEntry,
    EntryNotFound,
    InvalidCategory,
}

impl DataStore {
    pub fn delete_entry(&mut self, id: u32) -> Result<(), StorageError> {
        let pos = self.entries.iter().position(|e| e.id == id);
        
        match pos {
            Some(i) => {
                self.entries.remove(i);
                Ok(())
            }
            None => Err(StorageError::EntryNotFound),
        }
    }
    
    pub fn update_entry(&mut self, id: u32, value: f64) -> Result<(), StorageError> {
        self.entries
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or(StorageError::EntryNotFound)
            .map(|e| e.value = value)
    }
}
```

#### 4.4 Update main.rs with error handling
```rust
// Add to main.rs

fn delete_entry(store: &mut DataStore) {
    println!("\n--- Delete Entry ---");
    print!("Enter entry ID: ");
    
    if let Some(id) = cli::get_option(u32::MAX) {
        match store.delete_entry(id) {
            Ok(_) => println!("✓ Entry deleted!"),
            Err(e) => println!("✗ Error: {:?}", e),
        }
    } else {
        println!("Invalid ID");
    }
}
```

### Testing
```bash
cargo test
cargo run
```

### Exercises

**Exercise 4.1**: Add search function
```rust
pub fn search_entries(store: &DataStore, query: &str) -> Vec<&DataEntry> {
    store.get_entries()
        .iter()
        .filter(|e| e.title.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
```

**Exercise 4.2**: Add custom Result type
```rust
pub type DataResult<T> = Result<T, StorageError>;
```

**Exercise 4.3**: Add more error types
```rust
pub enum StorageError {
    InvalidEntry,
    EntryNotFound,
    InvalidCategory,
    InvalidValue,
    IOError(String),
}
```

**Exercise 4.4**: Create comprehensive error tests

### Milestone ✅
- ✅ Modular analysis functions
- ✅ Error handling with Result
- ✅ Pattern matching
- ✅ Better code organization

---

## 🗓️ DAY 5: File I/O & Persistence

**Duration**: 3-4 hours  
**Skills**: File operations, serialization, error handling

### Learning Objectives
- Read and write files
- Serialize/deserialize JSON
- Handle file errors
- Persist data

### Tasks

#### 5.1 Create persistence.rs
```rust
// src/persistence.rs
use crate::storage::DataStore;
use crate::models::DataEntry;
use std::fs;
use std::path::Path;
use std::io;

pub const DATA_FILE: &str = "data/entries.json";

pub fn ensure_data_dir() -> io::Result<()> {
    fs::create_dir_all("data")?;
    Ok(())
}

pub fn save_entries(store: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
    ensure_data_dir()?;
    
    let json = serde_json::to_string_pretty(store.get_entries())?;
    fs::write(DATA_FILE, json)?;
    
    println!("✓ Data saved successfully!");
    Ok(())
}

pub fn load_entries() -> Result<Vec<DataEntry>, Box<dyn std::error::Error>> {
    if !Path::new(DATA_FILE).exists() {
        return Ok(Vec::new());
    }
    
    let json = fs::read_to_string(DATA_FILE)?;
    let entries = serde_json::from_str(&json)?;
    
    println!("✓ Data loaded successfully!");
    Ok(entries)
}

pub fn backup_entries() -> Result<(), Box<dyn std::error::Error>> {
    ensure_data_dir()?;
    
    if !Path::new(DATA_FILE).exists() {
        return Ok(());
    }
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_file = format!("data/backup_{}.json", timestamp);
    fs::copy(DATA_FILE, backup_file)?;
    
    println!("✓ Backup created!");
    Ok(())
}

pub fn export_csv(store: &DataStore, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv = String::from("ID,Title,Category,Value,Timestamp,Notes\n");
    
    for entry in store.get_entries() {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            entry.id, entry.title, entry.category, entry.value, entry.timestamp, entry.notes
        ));
    }
    
    fs::write(filename, csv)?;
    println!("✓ Exported to CSV!");
    Ok(())
}
```

#### 5.2 Update storage.rs with persistence
```rust
// In storage.rs, add to DataStore

impl DataStore {
    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        use crate::persistence::load_entries;
        
        let entries = load_entries()?;
        let next_id = if entries.is_empty() {
            1
        } else {
            entries.iter().map(|e| e.id).max().unwrap_or(0) + 1
        };
        
        Ok(DataStore { entries, next_id })
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::persistence::save_entries;
        save_entries(self)
    }
}
```

#### 5.3 Update main.rs
```rust
// Update main function

fn main() {
    // ... intro ...
    
    // Load existing data
    let mut store = match DataStore::load_from_file() {
        Ok(store) => {
            println!("Loaded {} entries from file\n", store.get_entries().len());
            store
        }
        Err(_) => {
            println!("No existing data found. Starting fresh.\n");
            DataStore::new()
        }
    };
    
    let mut running = true;
    while running {
        show_menu();
        
        match cli::get_option(5) {
            Some(1) => add_entry(&mut store),
            Some(2) => view_dashboard(&store),
            Some(3) => export_data(&store),
            Some(4) => {
                if let Err(e) = store.save_to_file() {
                    eprintln!("Error saving: {}", e);
                }
                println!("Goodbye!");
                running = false;
            }
            _ => println!("Invalid option.\n"),
        }
    }
}

fn export_data(store: &DataStore) {
    println!("\n--- Export Options ---");
    println!("1. Export to JSON");
    println!("2. Export to CSV");
    println!("3. Create Backup\n");
    print!("Select option (1-3): ");
    
    match cli::get_option(3) {
        Some(1) => {
            if let Err(e) = store.save_to_file() {
                eprintln!("Error: {}", e);
            }
        }
        Some(2) => {
            if let Err(e) = crate::persistence::export_csv(store, "data/export.csv") {
                eprintln!("Error: {}", e);
            }
        }
        Some(3) => {
            if let Err(e) = crate::persistence::backup_entries() {
                eprintln!("Error: {}", e);
            }
        }
        _ => println!("Invalid option."),
    }
}
```

#### 5.4 Update lib.rs
```rust
// src/lib.rs (Updated)
pub mod models;
pub mod cli;
pub mod storage;
pub mod analysis;
pub mod persistence;
```

### Testing
```bash
cargo run
# Add some entries
# Exit to save
# Run again to load
```

### Exercises

**Exercise 5.1**: Add compression
```rust
pub fn save_compressed(store: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
    // Use flate2 crate for gzip
}
```

**Exercise 5.2**: Add data validation on load
```rust
pub fn validate_entry(entry: &DataEntry) -> bool {
    // Validate all fields
}
```

**Exercise 5.3**: Add file locking
```rust
pub fn safe_save(store: &DataStore) -> Result<(), Box<dyn std::error::Error>> {
    // Create .lock file
    // Save atomically
    // Remove .lock
}
```

**Exercise 5.4**: Write integration tests
```rust
#[test]
fn test_save_and_load() {
    // Create store, save, load, verify
}
```

### Milestone ✅
- ✅ File I/O working
- ✅ JSON serialization
- ✅ CSV export
- ✅ Data persistence

---

## 🗓️ DAY 6: WEEK 1 PROJECT - Complete CLI Dashboard

**Duration**: 4-5 hours  
**Skills**: Integration, testing, refinement

### Learning Objectives
- Integrate all Week 1 concepts
- Create complete working CLI
- Write comprehensive tests
- Polish user experience

### Tasks

#### 6.1 Enhance main menu
```rust
// src/main.rs - Full refactor

use personal_dashboard::{
    app_name, app_version,
    models::Category,
    storage::DataStore,
    cli,
};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║       {} {}       ║", app_name(), app_version());
    println!("║    Your Personal Data Hub            ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // Load existing data
    let mut store = match DataStore::load_from_file() {
        Ok(s) => {
            println!("✓ Loaded {} entries\n", s.get_entries().len());
            s
        }
        Err(_) => {
            println!("Starting fresh!\n");
            DataStore::new()
        }
    };
    
    let mut running = true;
    
    while running {
        show_main_menu();
        
        match cli::get_option(6) {
            Some(1) => handle_add_entry(&mut store),
            Some(2) => handle_view_dashboard(&store),
            Some(3) => handle_delete_entry(&mut store),
            Some(4) => handle_export_menu(&store),
            Some(5) => handle_analytics(&store),
            Some(6) => {
                if cli::confirm("Save before exiting?") {
                    let _ = store.save_to_file();
                }
                println!("\n✓ Goodbye!");
                running = false;
            }
            _ => println!("Invalid option. Try again.\n"),
        }
    }
}

fn show_main_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║           MAIN MENU                  ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ 1. ➕ Add Entry                       ║");
    println!("║ 2. 📊 View Dashboard                 ║");
    println!("║ 3. 🗑️  Delete Entry                  ║");
    println!("║ 4. 💾 Export Data                    ║");
    println!("║ 5. 📈 View Analytics                 ║");
    println!("║ 6. 🚪 Exit                           ║");
    println!("╚════════════════════════════════════════╝\n");
    print!("Select option (1-6): ");
}

fn handle_add_entry(store: &mut DataStore) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║       ADD NEW ENTRY                  ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // Get title
    print!("Title: ");
    let title = cli::read_user_input();
    
    if title.is_empty() {
        println!("✗ Title cannot be empty.\n");
        return;
    }
    
    // Get category
    println!("\nCategories:");
    for (i, cat) in Category::all().iter().enumerate() {
        println!("{}. {}", i + 1, cat);
    }
    print!("\nSelect category (1-5): ");
    
    let category = match cli::get_option(5) {
        Some(1) => Category::Health,
        Some(2) => Category::Finance,
        Some(3) => Category::Fitness,
        Some(4) => Category::Mood,
        Some(5) => Category::Other,
        _ => {
            println!("Invalid category.\n");
            return;
        }
    };
    
    // Get value
    print!("Value: ");
    let value = match cli::read_user_input().parse::<f64>() {
        Ok(v) => v,
        Err(_) => {
            println!("✗ Invalid value.\n");
            return;
        }
    };
    
    // Get notes
    print!("Notes (optional): ");
    let notes = cli::read_user_input();
    
    // Create entry
    use personal_dashboard::models::DataEntry;
    let entry = DataEntry::new(0, title, category, value, notes);
    store.add_entry(entry);
    
    println!("✓ Entry added!\n");
}

fn handle_view_dashboard(store: &DataStore) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║       DASHBOARD SUMMARY              ║");
    println!("╚════════════════════════════════════════╝\n");
    
    if store.get_entries().is_empty() {
        println!("No entries yet.\n");
        return;
    }
    
    println!("Total Entries: {}", store.get_entries().len());
    println!("Total Value: ${:.2}", store.total_value());
    println!("Average Value: ${:.2}\n", store.average_value());
    
    println!("--- By Category ---");
    let stats = store.get_category_stats();
    for cat in Category::all() {
        if let Some((count, sum, max)) = stats.get(&cat) {
            println!("{:<12} | Count: {:>2} | Sum: ${:>8.2} | Max: ${:>8.2}",
                     cat, count, sum, max);
        }
    }
    
    println!("\n--- Recent Entries ---");
    println!("{:<3} {:<20} {:<12} {:<10} {:<20}",
             "ID", "Title", "Category", "Value", "Date");
    println!("{:-<65}", "");
    
    for entry in store.get_entries().iter().rev().take(10) {
        println!("{:<3} {:<20} {:<12} ${:<9.2} {}",
                 entry.id,
                 truncate(&entry.title, 19),
                 entry.category,
                 entry.value,
                 entry.timestamp.format("%Y-%m-%d %H:%M"));
    }
    println!();
}

fn handle_delete_entry(store: &mut DataStore) {
    println!("\n--- Delete Entry ---");
    print!("Enter entry ID: ");
    
    if let Some(id) = cli::get_option(u32::MAX) {
        match store.delete_entry(id) {
            Ok(_) => println!("✓ Entry deleted!\n"),
            Err(_) => println!("✗ Entry not found.\n"),
        }
    }
}

fn handle_export_menu(store: &DataStore) {
    println!("\n--- Export Options ---");
    println!("1. Export to CSV");
    println!("2. Save to JSON");
    println!("3. Create Backup\n");
    print!("Select option (1-3): ");
    
    match cli::get_option(3) {
        Some(1) => {
            print!("Filename (default: export.csv): ");
            let filename = cli::read_user_input();
            let filename = if filename.is_empty() { "data/export.csv" } else { &filename };
            
            match crate::persistence::export_csv(store, filename) {
                Ok(_) => {},
                Err(e) => println!("✗ Error: {}", e),
            }
        }
        Some(2) => {
            if let Err(e) = store.save_to_file() {
                println!("✗ Error: {}", e);
            }
        }
        Some(3) => {
            if let Err(e) = crate::persistence::backup_entries() {
                println!("✗ Error: {}", e);
            }
        }
        _ => println!("Invalid option."),
    }
    println!();
}

fn handle_analytics(store: &DataStore) {
    use crate::analysis::{calculate_trend, categorize_by_range};
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║       ANALYTICS                      ║");
    println!("╚════════════════════════════════════════╝\n");
    
    println!("--- Trends ---");
    for cat in Category::all() {
        if let Some(trend) = calculate_trend(
            store.get_entries().iter().collect::<Vec<_>>().as_slice(),
            cat
        ) {
            let direction = if trend > 0.0 { "📈" } else { "📉" };
            println!("{} {}: {:.2}", direction, cat, trend);
        }
    }
    
    println!("\n--- By Value Range ---");
    let ranges = categorize_by_range(store);
    for (range, entries) in ranges {
        println!("{}: {} entries", range, entries.len());
    }
    println!();
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 2])
    } else {
        s.to_string()
    }
}
```

#### 6.2 Create comprehensive tests
```rust
// tests/integration_tests.rs
use personal_dashboard::{
    models::{DataEntry, Category},
    storage::DataStore,
};

#[test]
fn test_add_entry() {
    let mut store = DataStore::new();
    let entry = DataEntry::new(
        0,
        "Test".to_string(),
        Category::Health,
        50.0,
        "Notes".to_string(),
    );
    store.add_entry(entry);
    assert_eq!(store.get_entries().len(), 1);
}

#[test]
fn test_total_value() {
    let mut store = DataStore::new();
    for i in 1..=5 {
        let entry = DataEntry::new(
            i,
            format!("Entry {}", i),
            Category::Health,
            i as f64 * 10.0,
            "".to_string(),
        );
        store.add_entry(entry);
    }
    assert_eq!(store.total_value(), 150.0);
}

#[test]
fn test_filter_by_category() {
    let mut store = DataStore::new();
    
    store.add_entry(DataEntry::new(0, "H1".to_string(), Category::Health, 10.0, "".to_string()));
    store.add_entry(DataEntry::new(0, "F1".to_string(), Category::Finance, 20.0, "".to_string()));
    store.add_entry(DataEntry::new(0, "H2".to_string(), Category::Health, 30.0, "".to_string()));
    
    let health_entries = store.get_entries_by_category(Category::Health);
    assert_eq!(health_entries.len(), 2);
}

#[test]
fn test_delete_entry() {
    let mut store = DataStore::new();
    let entry = DataEntry::new(0, "Test".to_string(), Category::Other, 10.0, "".to_string());
    store.add_entry(entry);
    
    assert!(store.delete_entry(1).is_ok());
    assert_eq!(store.get_entries().len(), 0);
}
```

#### 6.3 Add documentation
```rust
// Add doc comments to all public items

//! Personal Dashboard
//!
//! A comprehensive data tracking application built with Rust.
//! 
//! # Features
//! - Add, view, and manage data entries
//! - Categorize entries for organization
//! - Export to CSV and JSON
//! - View analytics and trends
//! - Persistent file storage
```

### Testing & Refinement
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Build release version
cargo build --release
```

### Exercises

**Exercise 6.1**: Add configuration file
```rust
pub struct Config {
    pub data_dir: String,
    pub backup_enabled: bool,
}
```

**Exercise 6.2**: Add color output
- Add `colored` crate
- Color categories
- Color statistics

**Exercise 6.3**: Add search functionality
- Search by title
- Search by date range
- Search by value range

**Exercise 6.4**: Documentation tests
```rust
/// Creates a new entry
///
/// # Examples
/// ```
/// use personal_dashboard::models::{DataEntry, Category};
/// let entry = DataEntry::new(1, "Test".to_string(), Category::Health, 10.0, "".to_string());
/// assert_eq!(entry.value, 10.0);
/// ```
```

### WEEK 1 Milestone ✅
- ✅ Complete CLI application
- ✅ Data entry and management
- ✅ File persistence
- ✅ Analytics and filtering
- ✅ Error handling
- ✅ Comprehensive testing
- ✅ Professional code organization

---

## 📈 WEEK 2 OVERVIEW (Days 7-12)
**Focus**: Advanced collections, ownership, and data structures

### Day 7: Ownership & Borrowing (Deep Dive)
### Day 8: Advanced Collections (BTreeMap, LinkedList)
### Day 9: Custom Data Structures
### Day 10: Traits & Generics
### Day 11: Testing & Benchmarking
### Day 12: WEEK 2 PROJECT - Advanced Analytics Engine

---

## 🔄 WEEK 3 OVERVIEW (Days 13-18)
**Focus**: Memory management, lifetimes, and error handling

### Day 13: Lifetimes & References
### Day 14: Smart Pointers
### Day 15: Error Handling (Custom Types)
### Day 16: Type Safety
### Day 17: Performance Optimization
### Day 18: WEEK 3 PROJECT - Data Processing Engine

---

## 🚀 WEEK 4 OVERVIEW (Days 19-30)
**Focus**: Concurrency, networking, and deployment

### Days 19-21: Threading & Concurrency
### Days 22-24: Web Server (Tokio/Axum)
### Days 25-27: API Development
### Days 28-30: WEEK 4 PROJECT - Full-Featured Dashboard Server

---

## 📊 Complete Project Timeline

```
WEEK 1: CLI Dashboard (Days 1-6)
├─ Day 1: Project Setup
├─ Day 2: User Input
├─ Day 3: Collections
├─ Day 4: Functions
├─ Day 5: File I/O
└─ Day 6: Integration

WEEK 2: Analytics Engine (Days 7-12)
├─ Day 7: Ownership Deep Dive
├─ Day 8: Advanced Collections
├─ Day 9: Custom Structures
├─ Day 10: Traits & Generics
├─ Day 11: Testing & Benchmarks
└─ Day 12: Analytics Integration

WEEK 3: Data Processing (Days 13-18)
├─ Day 13: Lifetimes
├─ Day 14: Smart Pointers
├─ Day 15: Error Handling
├─ Day 16: Type Safety
├─ Day 17: Optimization
└─ Day 18: Processing Engine

WEEK 4: Web Services (Days 19-30)
├─ Days 19-21: Concurrency
├─ Days 22-24: Web Server
├─ Days 25-27: API Development
└─ Days 28-30: Full Integration

FINAL: Full-Stack Dashboard Application 🎉
```

---

## 🎯 Skills Progression

| Week | Phase | Skills | Output |
|------|-------|--------|--------|
| 1 | Fundamentals | CLI, I/O, Collections | Working CLI app |
| 2 | Ownership | Advanced types, traits | Analytics system |
| 3 | Advanced | Lifetimes, smart pointers | Data processor |
| 4 | Concurrency | Threading, async, web | Full server |

---

## 🏆 By Day 30, You'll Have

✅ **Complete Rust application** with 2000+ lines of code  
✅ **CLI interface** with user input and menus  
✅ **Data persistence** with JSON/CSV export  
✅ **Analytics engine** with statistics and trends  
✅ **Web API** serving dashboard data  
✅ **Concurrency handling** for performance  
✅ **Error handling** throughout  
✅ **Comprehensive tests** (100+ test cases)  
✅ **Full documentation**  
✅ **Production-ready code**  

---

## 💻 Running the Project

### Build
```bash
cargo build --release
```

### Run
```bash
./target/release/personal_dashboard
```

### Tests
```bash
cargo test --all
```

### Documentation
```bash
cargo doc --open
```

---

## 📚 Next Steps After Day 30

1. **Deploy** to production
2. **Add database** (SQLite, PostgreSQL)
3. **Build mobile client** (React/Flutter)
4. **Add authentication**
5. **Scale to cloud** (AWS, Azure)
6. **Open source** on GitHub
7. **Write blog post** about journey
8. **Contribute** to Rust ecosystem

---

This timeline creates a real, usable application while teaching you all core Rust concepts through practical experience!
