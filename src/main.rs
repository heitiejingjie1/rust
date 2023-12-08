fn main() {
    variable();
}

fn variable(){

    // 默认情况下，rust变量是不可变的
    let x = 5;
    println!("The value is:{x}");
    // x = 6;
    println!("The value is:{x}");

    // 要创建可更改的变量 使用 mut 关键字
    let mut y = 7;
    println!("The value is:{y}");
    y = 8;
    println!("The value is:{y}");

    // 常量，至始至终不可变的量，且使用关键字const声明，且声明时，只能是常量表达式
    const ONE_DAY_HOURS: u32  = 24;
    println!("hours = {ONE_DAY_HOURS}");

}
