use std::thread;
use std::sync::mpsc;

fn main() {
    // Definindo os números para calcular a soma
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Criando um canal para enviar resultados do thread de soma
    let (tx, rx) = mpsc::channel();

    // Movendo a propriedade dos números para dentro do loop
    let chunk_size = numbers.len() / 2; // Dividindo em duas partes

    // Utilizando 'move' para garantir que 'numbers' permaneça válido
    for chunk in numbers.chunks(chunk_size).map(|slice| slice.to_vec()) {
        let tx_clone = tx.clone(); // Clonando o transmissor para o thread
        
        // Movendo 'chunk' para dentro do thread com 'move'
        thread::spawn(move || {
            let sum: i32 = chunk.iter().sum(); // Calculando a soma dos números no chunk
            tx_clone.send(sum).unwrap(); // Enviando a soma pelo canal
        });
    }

    // Coletando os resultados dos threads
    let mut total_sum = 0;
    for _ in 0..2 { // Iterando duas vezes, pois temos dois threads
        total_sum += rx.recv().unwrap(); // Recebendo cada soma calculada
    }

    println!("A soma total dos números é: {}", total_sum);
}
