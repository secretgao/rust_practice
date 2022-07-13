#[derive(Debug)]
struct Rectangle{
    length:i32,   //长
    width:i32,  //宽
}
//定义trait
trait GraphArea{
    fn GetLength(&self)->i32;
    fn GetWidth(&self)->i32;
}
//实现trait
impl GraphArea for Rectangle{
    fn GetLength(&self)->i32{
        self.length
    }
    fn GetWidth(&self)->i32{
        self.width
    }
}
//打印输出面积
fn PrintGraphArea(item:impl GraphArea){
    println!("该图形的长度是:{},宽度是:{},面积是:{}",item.GetLength(),item.GetWidth(),item.GetLength()*item.GetWidth());
}
fn main(){
   let rectangle= Rectangle{width:44,length:50};
    PrintGraphArea(rectangle)
}
