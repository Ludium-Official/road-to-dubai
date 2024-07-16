mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}
    //현재 모듈 outermost, 그의 자식 모듈에서만 가능  
    mod inside {
        // inside는 현재 모듈인 outermost에서만 접근될 수 있다! 
        // 그리고 자식모듈 마저 없음 
        //따라서 아래에서 호출하는건 둘 다 오류가 생김
        pub fn inner_function() {}
        // 이건 inside를 pub로 만들면 Ok 

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}