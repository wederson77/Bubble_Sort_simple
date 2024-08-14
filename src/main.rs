// Função para trocar dois elementos de posição em um array
fn swap_elements(array: &mut [i32; 20], i: usize, j: usize) {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

fn main() {
    // Implementação do Bubble Sort: um algoritmo de ordenação simples que
    // percorre o array repetidamente, trocando elementos adjacentes que estão
    // fora de ordem. A complexidade do algoritmo é O(n^2).

    // Definição do array de 20 números inteiros
    let mut array: [i32; 20] = [42, 17, 56, 9, 87, 34, 65, 28, 74, 3, 93, 51, 32, 18, 41, 67, 12, 89, 7, 60];

    // Imprime o array inicial
    println!("Array inicial: {:?}", array);

    // Loop externo percorre cada elemento do array
    for i in 0..array.len() {
        // Loop interno percorre os elementos restantes, de trás para frente
        for j in ((i + 1)..array.len()).rev() {
            // Compara elementos adjacentes e troca se estiverem fora de ordem
            if array[j - 1] > array[j] {
                swap_elements(&mut array, j - 1, j);
            }
        }
        // Imprime o estado do array após cada iteração do loop externo
        println!("Estado atual do array: {:?}", array);
    }

    // Imprime o array final ordenado
    println!("Array ordenado: {:?}", array);
}
