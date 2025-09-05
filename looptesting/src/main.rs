


fn main() {
    let max_i: i32 = 5;
    let mut i: i32 = 0;

    let result = loop {
        i += 1;
        if i == max_i
        {
            //we return things via the break key word
            break i * 2;
        }
    };

    println!("Result of loop {}", result);


    //when dealing with loops, the break keyword will by default only break the highest level loop.
    //We can specify which loop to break by naming loops, and breaking them.

    let mut count = 8;
    
    //to name a loop, we need to add a ' char to its name in the front
    'count_loop: loop {
        println!("current count = {}", count);

        let mut remaining = 8;
        loop {
            
            if remaining == 5
            {
                break;
            }

            if count == 2
            {
                break 'count_loop
            }

            remaining -= 1;
        }

        count -= 1;

    }


    let mut i = 0;
    let max_i = 9;

    while i < max_i
    {
        println!("In while loop, i == {}", i);

        i += 1;
    }


    let elements = [1, 2, 3, 4, 5];

    for element in elements
    {
        println!("Current element = {}", element);
    }
}
