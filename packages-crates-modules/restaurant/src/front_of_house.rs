pub mod hosting; // move to its own file when size grows: src/front_of_house/hosting.rs

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }
    
        fn cook_order() {}
    }
}