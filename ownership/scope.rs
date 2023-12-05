fn main() {
    let s = "zzzhhhbbb";
    {
        let s = "aabbbcc";
        println!("{s}");
    }

    println!("{s}");


    let mut ss = String::from("hello");
    {
        let ss = String::from("helllo");
    }
    ss.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", ss);

}