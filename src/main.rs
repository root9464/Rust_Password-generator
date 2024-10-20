mod types;

use rand::seq::IteratorRandom;
use std::io::{self};
use termion::color;
use types::{default, ok, vk};

fn main() {
    let password: Option<_> = verified_password();
    match password {
        Some(password) => println!("{}Ваш пароль: {}", color::Fg(color::Green), password),
        None => println!("{}!!!ОШИБКА!!!", color::Fg(color::Red)),
    }
}

//Функция создания пароля
fn creating_password(long: usize) -> String {
    let mut rng = rand::thread_rng();
    let symbolss = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%&*;:?_-";
    let mut password = String::with_capacity(long);

    while password.len() != long {
        password.push(symbolss.chars().choose(&mut rng).unwrap());
    }
    password
}

//Функция проверки пароля
fn password_verification(
    long: usize,
    register_up: u8,
    register_low: u8,
    numbers: u8,
    special_char: u8,
    password: &str,
) -> bool {
    matches!(
        (
            checking_length(long, password),
            checking_register(long, register_up, register_low, password),
            checking_numbers(long, numbers, password),
            checking_special_char(special_char, password)
        ),
        (true, true, true, true)
    )
}

//Проверка длинны
fn checking_length(long: usize, password: &str) -> bool {
    if password.len() == long {
        return true;
    }
    false
}

//Проверка маленьких и больших букв
fn checking_register(long: usize, register_up: u8, register_low: u8, password: &str) -> bool {
    let mut capital_letters_up = 0;
    let mut capital_letters_low = 0;
    for i in 0..long {
        if password.as_bytes()[i].is_ascii_uppercase() {
            capital_letters_up += 1;
        }
        if password.as_bytes()[i].is_ascii_lowercase() {
            capital_letters_low += 1;
        }
    }
    (capital_letters_up >= register_up) && (capital_letters_low >= register_low)
}

//Проверка цифр
fn checking_numbers(long: usize, numbers: u8, password: &str) -> bool {
    let mut quantity_numbers = 0;
    for i in 0..long {
        if password.as_bytes()[i].is_ascii_digit() {
            quantity_numbers += 1;
        }
    }
    quantity_numbers >= numbers
}

//Проверка спец. символов
fn checking_special_char(special_char: u8, password: &str) -> bool {
    let mut quantity_special_char = 0;
    for i in password.as_bytes() {
        match i {
            b'!' | b'@' | b'#' | b'$' | b'%' | b'&' | b'*' | b';' | b':' | b'?' | b'_' | b'-' => {
                quantity_special_char += 1
            }
            _ => (),
        }
    }
    quantity_special_char >= special_char
}

//Выбор критериев пароля
fn sites_standards() -> Option<(usize, u8, u8, u8, u8)> {
    let mut site_name = String::new();

    println!("\nВыбирете сайт, для которого хотите сгенерировать пароль(введите цифру): \n1. VK \n2. OK \n3. Стандартный пароль\n");

    let _ = io::stdin().read_line(&mut site_name);

    match site_name.as_str().trim() {
        "1" => Some((
            vk().long,
            vk().register_up,
            vk().register_low,
            vk().numbers,
            vk().special_char,
        )),
        "2" => Some((
            ok().long,
            vk().register_up,
            ok().register_low,
            ok().numbers,
            ok().special_char,
        )),
        "3" => Some((
            default().long,
            default().register_up,
            default().register_low,
            default().numbers,
            default().special_char,
        )),
        _ => {
            println!(
                "\n{}Вы ввели неверное значение! Мы создадим пароль по дефольтным значениям.\n",
                color::Fg(color::Red)
            );
            Some((
                default().long,
                default().register_up,
                default().register_low,
                default().numbers,
                default().special_char,
            ))
        }
    }
}

//Создание пароля со всеми проверками
fn verified_password() -> Option<String> {
    let site_values: Option<(usize, u8, u8, u8, u8)> = sites_standards();
    match site_values {
        Some((long, register_up, register_low, numbers, special_char)) => {
            let mut password = creating_password(long);
            let mut verification = password_verification(
                long,
                register_up,
                register_low,
                numbers,
                special_char,
                &password.clone(),
            );
            if !verification {
                while !verification {
                    password = creating_password(long);
                    verification = password_verification(
                        long,
                        register_up,
                        register_low,
                        numbers,
                        special_char,
                        &password.clone(),
                    );
                }
            }
            Some(password)
        }
        None => None,
    }
}
