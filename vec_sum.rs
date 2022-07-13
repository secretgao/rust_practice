use rand::Rng;
use rand::random;
fn main(){
    let mut v:Vec<u32> =Vec::new(); //声明 类型是u32的vec集合
    let mut random = rand::thread_rng(); //声明随机变量
    for i in 0..11{
        v.push(random.gen_range(40,100)); //生成随机数 写入到集合中
    }
    println!("Vec: {:?}",v);
    println!("{:?}",sum(&v));
}
fn sum(item:&Vec<u32>) -> Option<u32>{
    let mut total:u32= 0;
    for i in item.into_iter(){
        total += i;
    }
    Some(total)
}
