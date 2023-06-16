// 定义一个函数类型别名，其中包含范型参数T
type OperationFn<T> = fn(T, T) -> T;

// 定义一个包装结构体，使得范型函数可以作为属性
struct OperationWrapper<T> {
    operation: Box<dyn Fn(T, T) -> T>,
}

impl<T: 'static> OperationWrapper<T> {
    fn new(operation: OperationFn<T>) -> Self {
        OperationWrapper {
            operation: Box::new(operation),
        }
    }
}

// 定义一个范型结构体，其中包含一个OperationWrapper属性
struct Calculator<T> {
    operation: OperationWrapper<T>,
}

impl<T: 'static> Calculator<T> {
    fn new(operation: OperationFn<T>) -> Self {
        Calculator {
            operation: OperationWrapper::new(operation),
        }
    }

    fn calculate(&self, a: T, b: T) -> T {
        (self.operation.operation)(a, b)
    }
}

// 定义一些简单的范型函数
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[test]
fn test1() {
    let calculator: Calculator<i32> = Calculator::new(add);
    println!("1 + 2 = {}", calculator.calculate(1, 2));
}
