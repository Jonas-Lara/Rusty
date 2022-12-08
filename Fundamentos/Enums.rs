//Las enumeraciones son un tipo de dato que permite definir sus posibles variantes

enum Movimiento {
    Arriba, //Variant
    Abajo,
    Izquierda,
    Derecha,
}

fn main() {
    let movimiento_jugador:Movimiento = Movimiento::Abajo;
    
    match movimiento_jugador {
        Movimiento::Arriba => println!("El jugador se mueve hacia arriba"),
        Movimiento::Abajo => println!("El jugador se mueve hacia abajo"),
        Movimiento::Izquierda => println!("El jugador se mueve hacia la izquierda"),
        Movimiento::Derecha => println!("El jugador se mueve hacia la derecha"),
    }
}