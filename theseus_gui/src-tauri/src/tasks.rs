use crate::state::State;

pub async fn do_task() {
        Box::pin(async move {
            let _state = State::get().await.unwrap();
        })
        .await;    
}

pub async fn do_task1()  {
    do_task().await;
}

pub async fn do_task2()  {
    do_task1().await;
}

pub async fn do_task3()  {
    do_task1().await;
    do_task2().await;
}

pub async fn do_task4()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
}

pub async fn do_task5()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
}

pub async fn do_task6()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
}

pub async fn do_task7()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
}

pub async fn do_task8()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
}

pub async fn do_task9()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
}

pub async fn do_task10()  {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
}

pub async fn do_task11() {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
    do_task10().await;
}

pub async fn do_task12() {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
    do_task10().await;
    do_task11().await;
}

pub async fn do_task13() {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
    do_task10().await;
    do_task11().await;
    do_task12().await;
}

pub async fn do_task14() {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
    do_task10().await;
    do_task11().await;
    do_task12().await;
    do_task13().await;
}

pub async fn do_task15() {
    do_task1().await;
    do_task2().await;
    do_task3().await;
    do_task4().await;
    do_task5().await;
    do_task6().await;
    do_task7().await;
    do_task8().await;
    do_task9().await;
    do_task10().await;
    do_task11().await;
    do_task12().await;
    do_task13().await;
    do_task14().await;
}