#![allow(unused)]
#![feature(linked_list_remove)] // `#![feature]` may not be used on the stable release channel
                                //rustup install nightly
                                //cargo +nightly install racer
                                //cargo +nightly test
                                //cargo +nightly run
                                //#![feature(repr128)]
                                //#[repr(u128)]
use nanoid::nanoid;
use std::collections::LinkedList;

extern crate getopts;
extern crate rustbox;
extern crate unicode_segmentation;

use getopts::Options;
use std::env;

use std::cmp::min;
use std::default::Default;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

use std::error::Error;

use rustbox::{Color, Key, RustBox};

extern crate term_size;

use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[derive(Clone)]
struct State {
    selected: usize,
    adding: String,
    //mode: Mode,
    status: Status,
    pass_input: String,
}

#[derive(Clone)]
enum Status {
    Dirty,
    Clean,
}
enum Movement {
    Top,
    Bottom,
    Up,
    Down,
}

#[derive(Clone)]
struct Pagina {
    nome_pagina: String,
    id: String,
    //indice: i32,
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
        //indice: 0,
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

fn alocar_novo_pagina(nome: String, lista: &mut LinkedList<Pagina>, l2: &mut LinkedList<Pagina>) {
    let b = build_pagina(nome, lista, l2);
    lista.push_back(b);
}

fn liberar_pagina(
    lista_liberando: &mut LinkedList<Pagina>,
    lista_recebendo: &mut LinkedList<Pagina>,
) {
    //let nome = String::from(&lista_alocada.front().clone().unwrap().nome_pagina);
    //lista_livre.push_back(build_Pagina(nome));
    if lista_liberando.is_empty() {
        println!("Requisitando liberação de página em uma Lista vazia!");
        return;
    }
    let b = lista_liberando.front_mut().cloned().unwrap();
    lista_recebendo.push_back(b);
    lista_liberando.pop_front();
}
fn alocar_pagina_exata(
    lista_liberando: &mut LinkedList<Pagina>,
    lista_recebendo: &mut LinkedList<Pagina>,
    pagina_nome: String,
) {
    if lista_liberando.is_empty() {
        println!("Requisitando liberação de página em uma Lista vazia!");
        return;
    }
    let mut flag = false;
    let ll = lista_liberando;
    let mut int = 0;
    for (i, pagina) in ll.iter().enumerate() {
        if pagina.nome_pagina == pagina_nome {
            let b = pagina.to_owned();
            lista_recebendo.push_back(b);
            int = i;
            flag = true;
        }
    }
    if !flag {
        println!("pagina inexistente!");
    } else {
        ll.remove(int);
    }
}

fn print_lista(lista: &mut LinkedList<Pagina>) -> &str {
    let mut s = String::new();
    s.push_str(" Inicio => ");
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
            s.push_str("] -> ");
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
fn main() {
    let mut paginas_alocadas: LinkedList<Pagina> = LinkedList::new();
    let mut paginas_livres: LinkedList<Pagina> = LinkedList::new();
    let pa = &mut paginas_alocadas;
    let pl = &mut paginas_livres;
    //alocando
    // alocar_novo_pagina("ei".to_string(), pa, pl);
    // alocar_novo_pagina("fala".to_string(), pa, pl);
    // alocar_novo_pagina("nada não".to_string(), pa, pl);
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);
    // //liberando
    // liberar_pagina(pa, pl);
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);
    // liberar_pagina(pl, pa);
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);
    // liberar_pagina(pa, pl);
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);
    // liberar_pagina(pa, pl);
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);

    // // liberando pagina exata
    // alocar_pagina_exata(pl, pa, "nada não".to_string());
    // println!("Páginas Alocadas: ");
    // print_lista(pa);
    // println!("Páginas Livres: ");
    // print_lista(pl);

    //Menu
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let bem_vindo = " Bem vindo ao Simulador de FIFO-buffer!! ";
    loop {
        rustbox.clear();
        if let Some((width, height)) = term_size::dimensions() {
            rustbox.print(
                (width - bem_vindo.len()) / 2,
                1,
                rustbox::RB_BOLD,
                Color::White,
                Color::Red,
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
            rustbox.print(
                1,
                4,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione '1' para alocar uma nova página.",
            );
            rustbox.print(
                1,
                5,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione '2' para liberar uma página das páginas alocadas. ",
            );
            rustbox.print(
                1,
                6,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione '3' para alocar uma página das páginas livre. ",
            );
            rustbox.print(
                1,
                7,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione '4' para alocar uma página exata das páginas livres. ",
            );
            rustbox.print(
                1,
                8,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                " Pressione 'Ctrl+c' para sair. ",
            );
            rustbox.print(
                1,
                10,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                "Páginas Alocadas: ",
            );
            rustbox.print(
                1,
                11,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                print_lista(pa),
            );
            rustbox.print(
                1,
                13,
                rustbox::RB_BOLD,
                Color::White,
                Color::Default,
                "Páginas Livres:  ",
            );
            rustbox.print(
                1,
                14,
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
                Key::Char('1') => {
                    //rustbox.set_input_mode(rustbox::InputMode::Alt);
                    let mut input = String::new();
                    rustbox.clear();
                    loop {
                        let width = rustbox.width();
                        rustbox.print(
                            (width - bem_vindo.len()) / 2,
                            1,
                            rustbox::RB_BOLD,
                            Color::White,
                            Color::Red,
                            bem_vindo,
                        );
                        rustbox.print(
                            1,
                            3,
                            rustbox::RB_BOLD,
                            Color::White,
                            Color::Default,
                            "Digite o nome: ",
                        );
                        rustbox.present();
                        match rustbox.poll_event(false) {
                            Ok(rustbox::Event::KeyEvent(key)) => match key {
                                Key::Ctrl('c') => {
                                    break;
                                }
                                Key::Enter => {
                                    let s = input.clone();
                                    alocar_novo_pagina(s, pa, pl);
                                    break;
                                }
                                Key::Backspace => {
                                    input.pop();
                                }
                                Key::Char(c) => {
                                    input.push(c);
                                }
                                _ => {
                                    input.clone();
                                }
                            },
                            Err(e) => panic!("{}", e),
                            _ => {}
                        }
                        rustbox.clear();
                        let input_str: &str = Box::leak(input.clone().into_boxed_str());
                        rustbox.print(
                            1,
                            4,
                            rustbox::RB_BOLD,
                            Color::White,
                            Color::Default,
                            input_str,
                        );
                        rustbox.present();
                    }
                }
                _ => {}
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
