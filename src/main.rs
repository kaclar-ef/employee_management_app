use std::collections::HashMap;
use std::io;

fn main() {
    let mut horses: HashMap<String, Vec<String>> = HashMap::new();
    horses.insert(
        String::from("短距離"),
        vec![
            String::from("カレンチャン"),
            String::from("サクラバクシンオー"),
        ],
    );
    horses.insert(
        String::from("中距離"),
        vec![
            String::from("シンボリルドルフ"),
            String::from("トウカイテイオー"),
        ],
    );

    loop {
        println!("メニューの番号を入力してください");
        println!("1：競走馬を追加する");
        println!("2：競走馬の一覧を見る");
        println!("3：終了");

        //値を選んでもらう
        let operation_num: u8 = chose_number();

        //選んでもらった数字によって必要な関数を動かす
        match operation_num {
            1 => {
                // 馬を追加する処理
                println!("追加したい距離を入力してください");
                let distance = input_text();
                println!("追加したい馬の名前を入力してください");
                let name = input_text();
                let names = horses.entry(distance).or_insert(vec![]);
                names.push(name);
                names.sort();
                println!("")
            }
            2 => {
                // 馬を表示する処理
                println!("見たい距離を選んでください");
                let len = horses.len();
                let mut count = 1;
                let mut distance_list = Vec::new();
                for (distance, _) in &horses {
                    distance_list.push(distance.clone());
                    println!("{}：{}", count, distance);
                    count += 1;
                }
                println!("{}：全ての馬を見る", len + 1);

                let operation_num: usize = chose_number() as usize;
                if operation_num == len + 1 {
                    print_all_horses(&horses);
                } else {
                    print_part_of_horses(operation_num, &horses, &distance_list);
                }
                println!("");
            }
            3 => break,
            _ => println!("無効な値です。再度選択してください。\n\n"),
        }
    }
}

// //ユーザーに数値を入力させる関数
fn chose_number() -> u8 {
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("入力に失敗しました");

    let num: u8 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return 255, //数字以外が入力された場合は255を返す
    };
    println!("");
    num
}

fn input_text() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力に失敗しました");
    input.pop();
    input
}

fn print_all_horses(horses: &HashMap<String, Vec<String>>) {
    for (distance, names) in horses {
        println!("{}馬一覧", distance);
        for name in names {
            println!("{}", name);
        }
        println!("");
    }
}

fn print_part_of_horses(
    num: usize,
    horses: &HashMap<String, Vec<String>>,
    distance_list: &Vec<String>,
) {
    let target_distance = &distance_list[num - 1];
    let names = horses.get(target_distance).unwrap();
    println!("{}馬一覧", target_distance);
    for name in names {
        println!("{}", name);
    }
}
