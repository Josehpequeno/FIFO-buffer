#![allow(unused)]
#![feature(linked_list_remove)]// `#![feature]` may not be used on the stable release channel
//rustup install nightly
//cargo +nightly install racer
//cargo +nightly test
//#![feature(repr128)]
//#[repr(u128)]            
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

fn alocar_novo_processo(nome: String, lista: &mut LinkedList<Bloco>) {
    let b = build_bloco(nome);
    lista.push_back(b);
}

fn liberar_processo(lista_liberando: &mut LinkedList<Bloco>, lista_recebendo: &mut LinkedList<Bloco>){
    //let nome = String::from(&lista_alocada.front().clone().unwrap().nome_processo);
    //lista_livre.push_back(build_bloco(nome));
    let b = lista_liberando.front_mut().cloned().unwrap();
    lista_recebendo.push_back(b);
    lista_liberando.pop_front();
}
fn liberar_processo_exato(lista_liberando: &mut LinkedList<Bloco>, lista_recebendo: &mut LinkedList<Bloco>, processo: String){
    let mut flag = false;
    let ll = lista_liberando;
    let mut int = 0;
    for (i, bloco) in ll.iter().enumerate() {
        if bloco.nome_processo == processo {
            lista_recebendo.push_back(*bloco);
            int = i;
            flag = true;
        }
    }
    if !flag {
        println!("Processo inexistente!");
    }else { 
        ll.remove(int);
    }
}


fn print_lista (lista: &mut LinkedList<Bloco>) {
    print!("Inicio => ");
    if lista.is_empty() {
        println!("Lista vazia!");
        return
    }
    for (i,bloco) in lista.iter().enumerate() {
        if i != lista.len()-1 {
            print!("|{}| -> ", bloco.nome_processo);
        }
        else {
            println!("|{}|", bloco.nome_processo);
        }
    }
}
fn main() {
    // let mut lista = Box::new(build_lista());
    // adiciona_bloco(String::from("ei"), &mut lista);
    // print_lista(lista);
    let mut paginas_alocadas: LinkedList<Bloco> = LinkedList::new();
    let mut paginas_livres: LinkedList<Bloco> = LinkedList::new();
    let pa = &mut paginas_alocadas;
    let pl = &mut paginas_livres;
    alocar_novo_processo(String::from("ei"), pa);
    alocar_novo_processo(String::from("fala"), pa);
    alocar_novo_processo(String::from("nada não"), pa);
    println!("Páginas Alocadas: ");
    print_lista(pa);
    println!("Páginas Alocadas: ");
    print_lista(pl);
    liberar_processo(pa, pl);
    println!("Páginas Alocadas: ");
    print_lista(pa);
    println!("Páginas Alocadas: ");
    print_lista(pl);
    liberar_processo(pl, pa);
    println!("Páginas Alocadas: ");
    print_lista(pa);
    println!("Páginas Alocadas: ");
    print_lista(pl);
}
