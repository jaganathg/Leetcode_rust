use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);

    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.56;
        println!("base2: {:?}", base2);
    }

    println!("****base: {:?}", base);

    
    let mut base3 = base.borrow_mut();
    base3.radio_freq += 45.13;

    println!("base3: {:?}", base3);
    println!("++++++base: {:?}", base);
}