use std::io::stdin;

#[derive(Copy, Clone)]
struct RW {
    wrt: i32,
    mutex: i32,
    rc: i32,
}

fn write(shared_data: &mut RW) -> RW {
    // 쓰기 작업과 mutex가 활성화 되어 있는 경우
    if shared_data.wrt == 1 && shared_data.mutex == 1 {
        println!("Writing"); // 쓰기 작업 시작
        shared_data.wrt = 0; // 작업 후 wrt의 flag를 0으로 변경
        shared_data.mutex = 0; // mutex의 flag를 0으로 설정
    } else if shared_data.rc > 0 {
        // 읽고 있는 사용자가 있는 경우
        println!("Some is reading do you wana stop T(1)|F(0) \n");
        let input = input();
        let t = match input {
            0 => false,
            _ => true,
        };
        if t {
            shared_data.rc = 0; // 사용자 초기화
            *shared_data = signal(shared_data); // 쓰기 신호 발생
        }
    } else {
        // 쓰기 작업이 진행 중이지 않을 경우
        println!("Some is writing do you wana stop T(1)|F(0) \n");
        let input = input();
        let t = match input {
            0 => false,
            _ => true,
        };
        if t {
            shared_data.rc = 0; // 사용자 초기화
            *shared_data = signal(shared_data); // 쓰기 신호 발생
        }
    }
    *shared_data
}

// 쓰기 신호를 보내는 함수
fn signal(shared_data: &mut RW) -> RW {
    // Writer는 동시에 쓰기 작업을 진행할 수 없음.
    // 동시에 쓰기 작업을 하지 않는 이유는 데이터의 일관성에 위배되기 때문.
    shared_data.wrt = 1;
    shared_data.mutex = 1;
    *shared_data
}

fn read(shared_data: &mut RW) -> RW {
    // Reader는 동시에 읽기 작업이 가능함.
    // 이유는 Reader에서는 데이터 수정이 발생하지 않아 데이터의 일관성이 유지되기 때문.
    shared_data.wrt = 0; // 쓰기 작업의 flag를 0으로 변경
    if shared_data.mutex == 1 {
        // mutex가 활성화 되어 있는 경우
        // 크리티컬 섹션에서 안전하게 작업을 함.
        shared_data.rc += 1; // read count 증가
        println!("{} Users are Reading\n", shared_data.rc); // 읽고 있는 유저의 수를 출력
    } else {
        // mutex가 활성화 되어 있지 않은 경우
        println!("Some is writing..");
    }
    *shared_data
}

fn input() -> u8 {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().parse::<u8>().unwrap();
    res
}

fn main() {
    let mut shared_data = RW {
        wrt: 1,
        mutex: 1,
        rc: 0,
    };
    loop {
        println!("Select an option \n");
        println!("1. Write \n");
        println!("2. Read \n");
        println!("3. Exit \n");
        let input = input();

        match input {
            1 => shared_data = write(&mut shared_data),
            2 => shared_data = read(&mut shared_data),
            _ => std::process::exit(0),
        };
    }
}
