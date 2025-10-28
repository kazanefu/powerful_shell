use rand::Rng;
use std::io::{self, Write};
use std::fs;
use std::env;
use chrono::prelude::*;

struct Shell {
    hp: i32,
    atk: i32,
    def: i32,
    grow: i32,
}

impl Shell {
    fn new() -> Self {
        Shell { hp: 10, atk: 2, def: 1, grow: 0 }
    }

    fn display(&self, turn: i32) {
        println!("=== ターン {}/10 ===", turn);
        println!("HP: {}  ATK: {}  DEF: {}  GROW: {}", self.hp, self.atk, self.def, self.grow);
    }

    fn random_event(&mut self) {
        let mut rng = rand::rng();
        let roll: u8 = rng.random_range(1..=6);
        match roll {
            1 => { self.hp -= 2; println!("強風が吹いた！HP -2"); },
            2 => { self.hp -= 3; println!("貝が病気になった！HP -3"); },
            3 => { self.atk += 2; println!("宝貝を発見！ATK +2"); },
            4 => { self.def += 2; println!("宝貝を発見！DEF +2"); },
            _ => println!("今日は平和な一日…"),
        }
    }

    fn save(&self) {
        let content = format!("hp:{} atk:{} def:{} grow:{}", self.hp, self.atk, self.def, self.grow);
        fs::write("shell_status.txt", content).expect("ファイル書き込み失敗");
        println!("hp:{} atk:{} def:{} grow:{}", self.hp, self.atk, self.def, self.grow);
    }

    fn load() -> Self {
        if let Ok(data) = fs::read_to_string("shell_status.txt") {
            let mut shell = Shell::new();
            for part in data.split_whitespace() {
                let kv: Vec<&str> = part.split(':').collect();
                if kv.len() != 2 { continue; }
                match kv[0] {
                    "hp" => shell.hp = kv[1].parse().unwrap_or(10),
                    "atk" => shell.atk = kv[1].parse().unwrap_or(2),
                    "def" => shell.def = kv[1].parse().unwrap_or(1),
                    "grow" => shell.grow = kv[1].parse().unwrap_or(0),
                    _ => {}
                }
            }
            shell
        } else {
            Shell::new()
        }
    }
    fn reset() {
        let shell = Shell::new();
        shell.save();
        println!("ステータスをリセットしました！");
    }
    fn show_path(){
        match fs::canonicalize("shell_status.txt") {
        Ok(path) => println!("ステータスファイルのパス: {}", path.display()),
        Err(_) => println!("ステータスファイルはまだ存在しません"),
    }
    }
    fn show_saved_status(){
        if let Ok(data) = fs::read_to_string("shell_status.txt") {
            println!("{}",data);
        }
    }
}

fn can_play_today() -> bool {
    let today = Local::now().date_naive();

    if let Ok(data) = fs::read_to_string("shell_status.txt") {
        for line in data.lines() {
            if line.starts_with("last_play:") {
                let last_play_str = line["last_play:".len()..].trim();
                if let Ok(last_play) = NaiveDate::parse_from_str(last_play_str, "%Y-%m-%d") {
                    return last_play != today; // 今日と違えばプレイ可能
                }
            }
        }
    }
    true // ファイルがない場合や last_play がなければプレイ可能
}
fn mark_played_today() {
    let today = Local::now().date_naive();
    let mut content = String::new();

    if let Ok(data) = fs::read_to_string("shell_status.txt") {
        for line in data.lines() {
            if line.starts_with("last_play:") {
                continue; // 古い日付は消す
            }
            content.push_str(line);
            content.push('\n');
        }
    }

    content.push_str(&format!("last_play:{}\n", today));
    fs::write("shell_status.txt", content).expect("書き込み失敗");
}

fn main() {

    if env::var("PSModulePath").is_err() {
        println!("このプログラムは PowerShell から実行してください。");
        return;
    }
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("コマンドを指定してください: play / reset / path / status");
        return;
    }

    match args[1].as_str() {
        "play" => {
            // ゲームプレイ関数呼び出し
            play();
        }
        "reset" => {
            // ステータス初期化関数呼び出し
            Shell::reset();
        }
        "path" => {
            // ステータスのテキストファイルのパスを表示
            Shell::show_path();
        }
        "status" => {
            // 保存済みのステータスを表示
            Shell::show_saved_status();
        }
        
        _ => println!("無効なコマンドです"),
    }

    
}

fn play(){
    if !can_play_today() {
        println!("今日はもうプレイ済みです。明日また来てください！");
        return;
    }
    println!("PowerShell から実行されています。ゲーム開始！");
    let mut shell = Shell::load();
    let mut input = String::new();

    for turn in 1..=10 {
        shell.display(turn);

        println!("行動を選んでください:");
        println!("1. 餌を与える  2. トレーニング  3. 防御練習  4. 休む");
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => { shell.hp += 2; shell.grow += 1; println!("餌を与えた！HP +2, 成長 +1"); },
            "2" => { shell.atk += 1; shell.hp -= 1; println!("トレーニングした！ATK +1, HP -1"); },
            "3" => { shell.def += 1; shell.hp -= 1; println!("防御練習した！DEF +1, HP -1"); },
            "4" => { shell.hp += 3; println!("休んだ！HP +3"); },
            _ => { println!("無効な入力です。何もしなかった。"); },
        }

        shell.random_event();

        if shell.hp <= 0 {
            println!("貝が死んでしまった…ゲームオーバー！");
            Shell::reset();
            return;
        }

        println!();
    }

    println!("=== 10ターン終了 ===");
    println!("最終ステータス: HP: {}  ATK: {}  DEF: {}  GROW: {}", shell.hp, shell.atk, shell.def, shell.grow);

    // シンプルな強さ判定
    let power = shell.hp + shell.atk*2 + shell.def*2 + shell.grow;
    println!("貝の総合力は {} です！", power);
    println!("あなたの貝は強かったでしょうか？");
    shell.save();
    mark_played_today();
}
