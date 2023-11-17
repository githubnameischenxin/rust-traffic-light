enum TrafficLight {
    Red(u32),
    Green(u32),
    Yellow(u32)
}

trait Duration {
    fn time(&self) -> &u32;
}

impl Duration for TrafficLight {
    fn time(&self) -> &u32 {
        match self {
            TrafficLight::Red(num) => num,
            TrafficLight::Green(num) => num,
            TrafficLight::Yellow(num) => num,
        }
    }
}

fn sum_list(list: &[u32]) -> Option<u32>{
    let mut sum = 0u32;
    for &num in list {
        match sum.checked_add(num) {
            Some(xx) => sum = xx,
            None => return None
        }
    }
    Some(sum)
}


trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64
}

struct Triangle {
    base: f64,
    height: f64
}

struct Square {
    side_length: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn name(&self) -> &str {
        "圆"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2f64
    }
    fn name(&self) -> &str {
        "三角形"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
    fn name(&self) -> &str {
        "正方形"
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("{:?} 面积为：{:?}", shape.name(), shape.area());
}

fn main() {
    let light = TrafficLight::Red(30);
    println!("红灯持续时间：{:?}s", light.time());

    let light = TrafficLight::Green(15);
    println!("绿灯持续时间：{:?}s", light.time());

    let light = TrafficLight::Yellow(3);
    println!("黄灯持续时间：{:?}s", light.time());

    let list = vec![1,2,3,4,5];
    let sum = sum_list(&list);
    println!("求和结果：{:?}", sum);

    let list = vec![1,2,3,4,5, u32::MAX];
    let sum = sum_list(&list);
    println!("求和结果溢出：{:?}", sum);

    let shape = Circle {
        radius: 6f64
    };
    print_area(shape);

    let shape = Triangle {
        base: 12f64,
        height: 5f64
    };
    print_area(shape);

    let shape = Square {
        side_length: 10f64
    };
    print_area(shape);

}
