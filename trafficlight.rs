
fn main(){
    let red = Trafficlight::Red;
    let yellow = Trafficlight::Yellow;
    let green = Trafficlight::Green;
    println!("Signal lamp color: {:?};time:{:?}s", red,red.red_time());
    println!("Signal lamp color: {:?};time:{:?}s", yellow,yellow.yellow_time());
    println!("Signal lamp color: {:?};time:{:?}s", green,green.green_time());
}
#[derive(Debug)]
enum Trafficlight{
    Red,
    Yellow,
    Green,
}

trait trafficlight{
    fn red_time(&self) ->u32;
    fn yellow_time(&self) ->u32;
    fn green_time(&self) ->u32;
}
impl trafficlight for Trafficlight{
    fn red_time(&self) ->u32{
        60
    }
    fn yellow_time(&self) ->u32{
        5
    }
    fn green_time(&self) ->u32{
        50
    }
}
