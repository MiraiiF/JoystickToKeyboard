use evdev::{Device, InputEventKind, Key, AbsoluteAxisType};
use enigo::{
    Direction::{Press, Release},
    Enigo, Keyboard, Key as EnigoKey, Settings,
};
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn create_button_map() -> (HashMap<Key, EnigoKey>, HashMap<Key, EnigoKey>){
    let mut button_map_first_joystick = HashMap::new();
    let mut button_map_second_joystick = HashMap::new();

    button_map_first_joystick.insert(Key::BTN_BASE6, EnigoKey::Unicode('o'));
    button_map_first_joystick.insert(Key::BTN_BASE2, EnigoKey::Unicode('l'));
    button_map_first_joystick.insert(Key::BTN_BASE5, EnigoKey::Unicode('p'));
    button_map_first_joystick.insert(Key::BTN_BASE, EnigoKey::Unicode('ç'));
    button_map_first_joystick.insert(Key::BTN_BASE4, EnigoKey::Unicode('k'));
    button_map_first_joystick.insert(Key::BTN_PINKIE, EnigoKey::Unicode('i'));
    button_map_first_joystick.insert(Key::BTN_BASE3, EnigoKey::Unicode('j'));
    button_map_first_joystick.insert(Key::BTN_TOP2, EnigoKey::Unicode('u'));
    button_map_first_joystick.insert(Key::BTN_THUMB2, EnigoKey::Unicode('n'));
    button_map_first_joystick.insert(Key::BTN_TOP, EnigoKey::Unicode('m'));

    button_map_second_joystick.insert(Key::BTN_BASE6, EnigoKey::Unicode('c'));
    button_map_second_joystick.insert(Key::BTN_BASE2, EnigoKey::Unicode('b'));
    button_map_second_joystick.insert(Key::BTN_BASE5, EnigoKey::Unicode('x'));
    button_map_second_joystick.insert(Key::BTN_BASE, EnigoKey::Unicode('v'));
    button_map_second_joystick.insert(Key::BTN_BASE4, EnigoKey::Unicode('g'));
    button_map_second_joystick.insert(Key::BTN_PINKIE, EnigoKey::Unicode('r'));
    button_map_second_joystick.insert(Key::BTN_BASE3, EnigoKey::Unicode('f'));
    button_map_second_joystick.insert(Key::BTN_TOP2, EnigoKey::Unicode('e'));
    button_map_second_joystick.insert(Key::BTN_THUMB2, EnigoKey::Unicode('q'));
    button_map_second_joystick.insert(Key::BTN_TOP, EnigoKey::Unicode('z'));

    (button_map_first_joystick, button_map_second_joystick)
}

fn processar_eventos(
    mut device: Device,
    button_map: HashMap<Key, EnigoKey>,
    axis_map: Vec<EnigoKey>,
    pilha: Arc<Mutex<Vec<EnigoKey>>>
) -> std::io::Result<()> {
    loop {
        for ev in device.fetch_events()? {
            match ev.kind() {
                InputEventKind::Key(key) => {
                    if let Some(&enigo_key) = button_map.get(&key) {
                        if ev.value() == 1 {
                            
                            let mut minha_pilha = pilha.lock().unwrap();
                            minha_pilha.push(enigo_key);
                        }
                    }
                }
                InputEventKind::AbsAxis(axis) => {

                    let mut minha_pilha = pilha.lock().unwrap();
                    match axis{
                        AbsoluteAxisType::ABS_X =>{
                            if ev.value() == 255 || ev.value() == 32767{
                                minha_pilha.push(axis_map[3].clone());
                            }
                            else if ev.value() == 0 || ev.value() == -32768{
                                minha_pilha.push(axis_map[1].clone());
                            }
                        }
                        AbsoluteAxisType::ABS_Y =>{
                            if ev.value() == 255 || ev.value() == 32767{
                                minha_pilha.push(axis_map[2].clone());
                            }
                            else if ev.value() == 0 || ev.value() == -32768{
                                minha_pilha.push(axis_map[0].clone());
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let first_joystick_device = Device::open("/dev/input/event19")?;
    let second_joystick_device = Device::open("/dev/input/event20")?;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let pilha:Arc<Mutex<Vec<EnigoKey>>> = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];

    let (button_map_first_joystick, button_map_second_joystick) = create_button_map();

    let mut first_joystick_axis = vec![];
    let mut second_joystick_axis = vec![];

    first_joystick_axis.push(EnigoKey::Unicode('w'));
    first_joystick_axis.push(EnigoKey::Unicode('a'));
    first_joystick_axis.push(EnigoKey::Unicode('s'));
    first_joystick_axis.push(EnigoKey::Unicode('d'));
    second_joystick_axis.push(EnigoKey::UpArrow);
    second_joystick_axis.push(EnigoKey::LeftArrow);
    second_joystick_axis.push(EnigoKey::DownArrow);
    second_joystick_axis.push(EnigoKey::RightArrow);

    let pilha_1_clone = Arc::clone(&pilha);
    let handle = thread::spawn(move || {
        processar_eventos(first_joystick_device, button_map_first_joystick, first_joystick_axis, pilha_1_clone).unwrap();
    });
    handles.push(handle);

    let pilha_2_clone = Arc::clone(&pilha);
    let handle = thread::spawn(move || {
        processar_eventos(second_joystick_device, button_map_second_joystick, second_joystick_axis, pilha_2_clone).unwrap();
    });
    handles.push(handle);

    loop{
        let mut pilha_real = pilha.lock().unwrap();
        let hash:HashSet<_> = pilha_real.drain(..).collect();
        pilha_real.extend(hash.into_iter());

        for elemento in pilha_real.iter(){
            enigo.key(elemento.clone(), Press).unwrap();
        }
        thread::sleep(Duration::from_millis(1));
        for elemento in pilha_real.iter(){
            enigo.key(elemento.clone(), Release).unwrap();
        }
        pilha_real.clear();
        drop(pilha_real);
        thread::sleep(Duration::from_millis(10));
    }
}
