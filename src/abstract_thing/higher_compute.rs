


use timely::dataflow::operators::{ToStream, Map, Inspect};

#[test]
fn test_f1() {
    // 初始化并启动计算
    timely::execute_directly(move |worker| {
        // 在给定的计算作用域创建一个数据流
        worker.dataflow::<u32,_,_>(|scope| {
            // 创建一个由0到9的数字流
            (0..10).to_stream(scope)
                // 将每个元素乘以2
                .map(|x| x * 2)
                // 输出每个元素
                .inspect(|x| println!("seen: {:?}", x));
        });
    });
}


use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};

#[test]
fn test_line() {
    let mut model = Model::new("Readme example", Objective::Max);
    let mut vars: Vec<Var> = vec![];

    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(5.0));

    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(2.0, &vars[1])],
        Operator::Le,
        170.0,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(1.0, &vars[1])],
        Operator::Le,
        150.0,
    );
    model.reg_constr(
        vec![Summand(0.0, &vars[0]), Summand(3.0, &vars[1])],
        Operator::Le,
        180.0,
    );

    model.optimize();
    print!("{}", model);


}
