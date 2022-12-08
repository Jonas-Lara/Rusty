fn main()
{
    //Loops for
    let animales = ["perro", "gato", "conejo", "pajaro"];

    for (index, a) in animales.iter().enumerate()
    {
        println!("El indice {} corresponde al animal {}", index, a);
    }

    //Loops infinitos
    let mut n = 0;

    loop
    {
        n +=1;
        
        if n == 10
        {
            break;
        }

        println!("n = {}", n);
    }

    //Loops while
    let mut n = 0;

    while n <= 50
    {
        if n % 5 == 0
        {
            println!("n = {}", n);    
        }

        n += 1;
    }
}