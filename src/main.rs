use std::collections::LinkedList;
#[derive(Clone)]
struct Bloco {
    nome_processo: String,
    //indice: i32,
    //proximo: Option<Box<Bloco>>,//  Box -> senão o Bloco vai ao infinito.
}

fn build_bloco (nome: String) -> Bloco {
    let b = Bloco {
        nome_processo: nome,
        //indice: 0,
        //proximo: None,
    };
    b    
}

// struct Lista {
//     tamanho: i32,
//     primeiro: Option<Box<Bloco>>,
//     ultimo: Option<Box<Bloco>>,
// }
// fn build_lista () -> Lista {
//     let l = Lista {
//         tamanho: 0,
//         primeiro: None,
//         ultimo: None,
//     };
//     l
// }
// fn adiciona_bloco (nome: String, lista: &mut Lista) {
//     let mut b = build_bloco(nome);
//     if lista.tamanho == 0 {
//         b.indice = 0;
//         lista.primeiro = Some(Box::new(b));
//         lista.ultimo = lista.primeiro;
//         lista.tamanho +=1;
//     } else {
//         b.indice = lista.tamanho;
//         match lista.ultimo { 
//             Some(x) => match x.proximo {
//                 Some(mut y) => {
//                     y = Box::new(b);
//                     lista.ultimo = Some(y);
//                 },
//                 None => print!("Err"),
//             },
//             None => print!("Err"),
//         }
//         lista.tamanho +=1;
//     }  
// }
// fn print_lista (lista: Box<Lista>) {
//     if lista.tamanho == 0 {
//         println!()
//     }
// }

fn alocar_novo_processo(nome: String, lista: &mut LinkedList<Bloco>) {
    let mut b = build_bloco(nome);
    lista.push_back(b);
}

fn liberar_processo(listaAlocada: &mut LinkedList<Bloco>, listaLivre: &mut LinkedList<Bloco>){
    let mut l = *listaAlocada.back().clone().unwrap();
    listaLivre.push_back(l);
    listaAlocada.pop_back();
}

fn print_lista (lista: LinkedList<Bloco>) {
    for (i,bloco) in lista.iter().enumerate() {
        if i != lista.len()-1 {
            print!("{} -> ", bloco.nome_processo);
        }
        else {
            println!("{}", bloco.nome_processo);
        }
    }
}
fn main() {
    // let mut lista = Box::new(build_lista());
    // adiciona_bloco(String::from("ei"), &mut lista);
    // print_lista(lista);
    let mut paginas_alocadas: LinkedList<Bloco> = LinkedList::new();
    let mut paginas_livres: LinkedList<Bloco> = LinkedList::new();
    alocar_novo_processo(String::from("ei"), &mut paginas_alocadas);
    alocar_novo_processo(String::from("fala"), &mut paginas_alocadas);
    alocar_novo_processo(String::from("nada não"), &mut paginas_alocadas);
    println!("Páginas Alocadas: ");
    print_lista(paginas_alocadas);
    println!("Páginas Alocadas: ");
    print_lista(paginas_livres)
}
