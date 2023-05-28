use std::env;
use std::fs;
mod cliArgs;
mod config;



fn main() {
    // 解析命令行参数，并返回一个 Args 实例
    let args = cliArgs::index::Args::parse();

    // 从 Args 实例中获取 name 和 count 的值
    let name = args.name;
    let count = args.count;

    // 根据 count 的值，打印出相应次数的问候语
    for _ in 0..count {
        println!("Hello {}!", name);
    }
    // // 1. 参数处理, 处理用户输入
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);


    // let query = args.get(1).expect("missing query args");
    // let file_path = args.get(2).expect("missing file_path args");
    // config::index::Config::of(
    //     &query,
    //     &file_path
    // );
    // println!("Searching for {query} in file {file_path}");

    // // 2. 文件读取
    // // ======
    // let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // // println!("With text in {file_path}:\n{content}");
    
    // // 3. 搜索
    // // ======
    // let mut line_index = -1;
    // let search_result = content.lines().map(|line| {
    //     line_index += 1;
    //     if line.contains(query) {
    //         return (
    //             line_index, line.find(query)
    //         )
    //     } else {
    //         return (
    //             line_index, None
    //         )
    //     }
    // }).filter(|(_, line)| {
    //     line.is_some()
    // });
    
    // // 4. 输出
    // let mut exist = false;
    // search_result.for_each(|(line, index)| {
    //     exist = true;
    //     println!("Searching Result in content: line: {line} index {}", index.unwrap());
    // });
    // if !exist {
    //     println!("Not Found in content")
    // }

    // println!("Hello, world!");
}
