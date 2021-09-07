#![feature(linked_list_remove)]
extern crate rustbox;
extern crate term_size;

use nanoid::nanoid;

use std::collections::LinkedList;
use std::default::Default;

use rustbox::{Color, Key, RustBox};

#[derive(Clone)]
struct Pagina {
    nome_pagina: String,
    id: String,
    //proximo: Option<Box<Pagina>>,//  Box -> senão o Pagina vai ao infinito.
}

fn build_pagina(nome: String, l1: &mut LinkedList<Pagina>, l2: &mut LinkedList<Pagina>) -> Pagina {
    let mut i = 4;
    let alphabet: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut id = nanoid!(i, &alphabet);
    let mut id_ref = &id;
    while busca_id(id_ref, l1, l2) {
        i += 1;
        id = nanoid!(i, &alphabet);
        id_ref = &id;
    }
    let b = Pagina {
        nome_pagina: nome,
        id: id,
        //proximo: None,
    };
    b
}

fn busca_id(id: &String, l1: &mut LinkedList<Pagina>, l2: &mut LinkedList<Pagina>) -> bool {
    let mut flag = false;
    for pagina in l1.iter() {
        if pagina.id == *id {
            flag = true;
        }
    }
    for pagina in l2.iter() {
        if pagina.id == *id {
            flag = true;
        }
    }
    flag
}

fn alocar_nova_pagina<'a>(
    nome: String,
    lista: &'a mut LinkedList<Pagina>,
    l2: &mut LinkedList<Pagina>,
) -> (bool, &'a str) {
    if nome.is_empty() {
        return (true, "✗ Não pode alocar uma página sem nome!");
    }
    let b = build_pagina(nome, lista, l2);
    lista.push_back(b);
    (false, "✓ Nova página alocada!")
}

fn liberar_pagina<'a>(
    lista_liberando: &mut LinkedList<Pagina>,
    lista_recebendo: &'a mut LinkedList<Pagina>,
    alocar: bool,
) -> (bool, &'a str) {
    if lista_liberando.is_empty() {
        if !alocar {
            return (
                true,
                "✗ Requisitando liberação da lista de páginas alocadas. Essa Lista está vazia!",
            );
        } else {
            return (
                true,
                "✗ Requisitando liberação da lista de páginas livres. Essa Lista está vazia!",
            );
        }
    }
    let b = lista_liberando.front_mut().cloned().unwrap();
    lista_recebendo.push_back(b);
    lista_liberando.pop_front();
    if alocar {
        (false, "✓ Página alocada!")
    } else {
        (false, "✓ Página liberada!")
    }
}
fn alocar_pagina_exata<'a>(
    pagina_id: String,
    lista_liberando: &'a mut LinkedList<Pagina>,
    lista_recebendo: &mut LinkedList<Pagina>,
) -> (bool, &'a str) {
    if lista_liberando.is_empty() {
        return (
            true,
            "✗ Requisitando liberação da lista de páginas livres. Essa Lista está vazia!",
        );
    }
    let mut flag = false;
    let ll = lista_liberando;
    let mut int = 0;
    for (i, pagina) in ll.iter().enumerate() {
        if pagina.id == pagina_id {
            let b = pagina.to_owned();
            lista_recebendo.push_back(b);
            int = i;
            flag = true;
        }
    }
    if !flag {
        (!flag, "✗ Página inexistente!")
    } else {
        ll.remove(int);
        let mut string = String::new();
        let p_id: &str = Box::leak(pagina_id.into_boxed_str());
        string.push_str("✓ Página ");
        string.push_str(p_id);
        string.push_str(" alocada!");
        let string_slice: &str = Box::leak(string.into_boxed_str());
        (!flag, string_slice)
    }
}

fn print_lista(lista: &mut LinkedList<Pagina>) -> &str {
    let mut s = String::new();
    s.push_str(" Inicio 🡆 ");
    if lista.is_empty() {
        s.push_str("Lista vazia!");
        let s_slice: &str = Box::leak(s.into_boxed_str());
        return s_slice; // convertendo String em &str
    }
    for (i, pagina) in lista.iter().enumerate() {
        if i != lista.len() - 1 {
            s.push_str("[");
            s.push_str(&pagina.nome_pagina[..]);
            s.push_str(" | id: ");
            s.push_str(&pagina.id[..]);
            s.push_str("] ⟼  ");
        } else {
            s.push_str("[");
            s.push_str(&pagina.nome_pagina[..]);
            s.push_str(" | id: ");
            s.push_str(&pagina.id[..]);
            s.push_str("]");
        }
    }
    Box::leak(s.into_boxed_str())
}

fn menu_change(rustbox: &RustBox, option: i32) {
    match option {
        0 => {
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                " ‣ Alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione 'Ctrl+c' para sair/voltar. ",
            );
            rustbox.present();
        }
        1 => {
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                " ‣ Liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione 'Ctrl+c' para sair/voltar. ",
            );
            rustbox.present();
        }
        2 => {
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                " ‣ Alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione 'Ctrl+c' para sair/voltar. ",
            );
            rustbox.present();
        }
        3 => {
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                " ‣ Alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione 'Ctrl+c' para sair/voltar. ",
            );
            rustbox.present();
        }
        4 => {
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                " ‣ Pressione 'Ctrl+c' para sair/voltar. ",
            );
            rustbox.present();
        }
        _ => {}
    }
}
fn main() {
    let mut paginas_alocadas: LinkedList<Pagina> = LinkedList::new();
    let mut paginas_livres: LinkedList<Pagina> = LinkedList::new();
    let pa = &mut paginas_alocadas;
    let pl = &mut paginas_livres;
    //Menu
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let bem_vindo = " Bem vindo ao Simulador de FIFO-buffer!! ";
    let mut option = 0;
    loop {
        if let Some((width, _height)) = term_size::dimensions() {
            rustbox.print(
                (width - bem_vindo.len()) / 2,
                1,
                rustbox::RB_BOLD,
                Color::White,
                Color::Blue,
                bem_vindo,
            );
            rustbox.print(
                1,
                3,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                "Escolha uma das Opções abaixo:  ",
            );
            menu_change(&rustbox, option);
            rustbox.print(
                1,
                12,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                "Páginas Alocadas: ",
            );
            rustbox.print(
                1,
                13,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                print_lista(pa),
            );
            rustbox.print(
                1,
                15,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                "Páginas Livres:  ",
            );
            rustbox.print(
                1,
                16,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                print_lista(pl),
            );
        }
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                Key::Ctrl('c') => {
                    break;
                }
                Key::Down => {
                    if option != 4 {
                        option += 1;
                        rustbox.clear();
                        menu_change(&rustbox, option);
                    }
                }
                Key::Up => {
                    if option != 0 {
                        option -= 1;
                        rustbox.clear();
                        menu_change(&rustbox, option);
                    }
                }
                Key::Enter => match option {
                    0 => {
                        let mut input = String::new();
                        input.push('▉');
                        rustbox.clear();
                        loop {
                            let width = rustbox.width();
                            rustbox.print(
                                (width - bem_vindo.len()) / 2,
                                1,
                                rustbox::RB_BOLD,
                                Color::White,
                                Color::Blue,
                                bem_vindo,
                            );
                            rustbox.print(
                                1,
                                4,
                                rustbox::RB_BOLD,
                                Color::White,
                                Color::Default,
                                "Digite o nome: ",
                            );
                            let input_s: &str = Box::leak(input.clone().into_boxed_str());
                            rustbox.print(
                                1,
                                5,
                                rustbox::RB_BOLD,
                                Color::White,
                                Color::Default,
                                input_s,
                            );
                            rustbox.present();
                            match rustbox.poll_event(false) {
                                Ok(rustbox::Event::KeyEvent(key)) => match key {
                                    Key::Ctrl('c') => {
                                        break;
                                    }
                                    Key::Enter => {
                                        input.pop();
                                        let s = input.clone();
                                        let (err, err_msg) = alocar_nova_pagina(s, pa, pl);
                                        if err {
                                            rustbox.clear();
                                            rustbox.print(
                                                1,
                                                3,
                                                rustbox::RB_BOLD,
                                                Color::Red,
                                                Color::Default,
                                                err_msg,
                                            );
                                            rustbox.present();
                                            input.push('▉');
                                            continue;
                                        } else {
                                            rustbox.print(
                                                1,
                                                10,
                                                rustbox::RB_BOLD,
                                                Color::Green,
                                                Color::Default,
                                                err_msg,
                                            );
                                            rustbox.present();
                                            break;
                                        }
                                    }
                                    Key::Backspace => {
                                        input.pop();
                                        input.pop();
                                        input.push('▉');
                                    }
                                    Key::Char(c) => {
                                        input.pop();
                                        input.push(c);
                                        input.push('▉');
                                    }
                                    _ => {}
                                },
                                Err(e) => panic!("{}", e),
                                _ => {}
                            }
                            rustbox.clear();
                            let input_str: &str = Box::leak(input.clone().into_boxed_str());
                            rustbox.print(
                                1,
                                5,
                                rustbox::RB_BOLD,
                                Color::White,
                                Color::Default,
                                input_str,
                            );
                            rustbox.present();
                        }
                    }
                    1 => {
                        let (err, err_msg) = liberar_pagina(pa, pl, false);
                        if err {
                            rustbox.clear();
                            rustbox.print(
                                1,
                                10,
                                rustbox::RB_BOLD,
                                Color::Red,
                                Color::Default,
                                err_msg,
                            );
                            rustbox.present();
                        } else {
                            rustbox.clear();
                            rustbox.print(
                                1,
                                10,
                                rustbox::RB_BOLD,
                                Color::Green,
                                Color::Default,
                                err_msg,
                            );
                            rustbox.present();
                        }
                    }
                    2 => {
                        let (err, err_msg) = liberar_pagina(pl, pa, true);
                        if err {
                            rustbox.clear();
                            rustbox.print(
                                1,
                                10,
                                rustbox::RB_BOLD,
                                Color::Red,
                                Color::Default,
                                err_msg,
                            );
                            rustbox.present();
                        } else {
                            rustbox.clear();
                            rustbox.print(
                                1,
                                10,
                                rustbox::RB_BOLD,
                                Color::Green,
                                Color::Default,
                                err_msg,
                            );
                            rustbox.present();
                        }
                    }
                    3 => {
                        let mut input = String::new();
                        input.push('▉');
                        if pl.is_empty() {
                            let (_err, err_msg) = alocar_pagina_exata("".to_string(), pl, pa);
                            rustbox.print(
                                1,
                                10,
                                rustbox::RB_BOLD,
                                Color::Red,
                                Color::Default,
                                err_msg,
                            );
                            rustbox.present();
                        } else {
                            rustbox.clear();
                            loop {
                                let width = rustbox.width();
                                rustbox.print(
                                    (width - bem_vindo.len()) / 2,
                                    1,
                                    rustbox::RB_BOLD,
                                    Color::White,
                                    Color::Blue,
                                    bem_vindo,
                                );
                                rustbox.print(
                                    1,
                                    4,
                                    rustbox::RB_BOLD,
                                    Color::White,
                                    Color::Default,
                                    "Digite o id da página: ",
                                );
                                let input_s: &str = Box::leak(input.clone().into_boxed_str());
                                rustbox.print(
                                    1,
                                    5,
                                    rustbox::RB_BOLD,
                                    Color::White,
                                    Color::Default,
                                    input_s,
                                );
                                rustbox.present();
                                match rustbox.poll_event(false) {
                                    Ok(rustbox::Event::KeyEvent(key)) => match key {
                                        Key::Ctrl('c') => {
                                            break;
                                        }
                                        Key::Enter => {
                                            input.pop();
                                            let s = input.clone();
                                            let (err, err_msg) = alocar_pagina_exata(s, pl, pa);
                                            if err {
                                                rustbox.print(
                                                    1,
                                                    3,
                                                    rustbox::RB_BOLD,
                                                    Color::Red,
                                                    Color::Default,
                                                    err_msg,
                                                );
                                                rustbox.present();
                                                input.push('▉');
                                                continue;
                                            } else {
                                                rustbox.print(
                                                    1,
                                                    10,
                                                    rustbox::RB_BOLD,
                                                    Color::Green,
                                                    Color::Default,
                                                    err_msg,
                                                );
                                                rustbox.present();
                                                break;
                                            }
                                        }
                                        Key::Backspace => {
                                            input.pop();
                                            input.pop();
                                            input.push('▉');
                                        }
                                        Key::Char(c) => {
                                            input.pop();
                                            input.push(c);
                                            input.push('▉');
                                        }
                                        _ => {}
                                    },
                                    Err(e) => panic!("{}", e),
                                    _ => {}
                                }
                                rustbox.clear();
                                let input_str: &str = Box::leak(input.clone().into_boxed_str());
                                rustbox.print(
                                    1,
                                    5,
                                    rustbox::RB_BOLD,
                                    Color::White,
                                    Color::Default,
                                    input_str,
                                );
                                rustbox.present();
                            }
                        }
                    }
                    4 => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
