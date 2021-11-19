use std::collections::HashMap;
use std::io;

fn main() {
    let mut horses: HashMap<String, Vec<String>> = HashMap::new();
    horses.insert(
        String::from("短距離"),
        vec![
            String::from("サクラバクシンオー"),
            String::from("カレンチャン"),
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
                println!("追加したい距離を入力してください");
                let distance = input_text();
                println!("追加したい馬の名前を入力してください");
                let name = input_text();
                horses.entry(distance).or_insert(vec![]).push(name);
                println!("{:?}", horses);
            } // 1 => print_average(&numbers),
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

