trait Layer1 {
    fn layer1(&self);
}

trait Layer2: Layer1 {
    fn layer2(&self);
}

trait Layer3: Layer2 {
    fn layer3(&self);
}

struct ImplLayer1;

impl Layer1 for ImplLayer1 {
    fn layer1(&self) {
        println!("Layer 1");
    }
}

struct ImplLayer2 {
    layer1: ImplLayer1,
}

impl Layer1 for ImplLayer2 {
    fn layer1(&self) {
        self.layer1.layer1();
    }
}

impl Layer2 for ImplLayer2 {
    fn layer2(&self) {
        println!("Layer 2");
    }
}

struct ImplLayer3 {
    layer2: ImplLayer2,
}

//多层继承，使用的把特征都赋予结构体形成的，组合的形式。一个结构体的属性持有另一个结构体的句柄
impl Layer1 for ImplLayer3 {
    fn layer1(&self) {
        self.layer2.layer1();
    }
}

impl Layer2 for ImplLayer3 {
    fn layer2(&self) {
        self.layer2.layer2();
    }
}

impl Layer3 for ImplLayer3 {
    fn layer3(&self) {
        println!("Layer 3");
    }
}

#[test]
fn test_7() {
    let layer3 = ImplLayer3 {
                        layer2: ImplLayer2 {
                            layer1: ImplLayer1,
                        },
                    };

    layer3.layer1();
    layer3.layer2();
    layer3.layer3();

}
