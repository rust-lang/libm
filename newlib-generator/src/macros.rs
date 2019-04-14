macro_rules! f32 {
    ($($fun:ident,)+) => {
        $(
            let fun = stringify!($fun);

            fs::create_dir_all("math/src")?;
            let run = format!("
fn run() -> Result<(), ()> {{
    
    #[link(name=\"m\")]
    extern \"C\" {{
        fn {0}(_: f32) -> f32;
    }}
    
    let mut stdout = hio::hstdout()?;
    let mut buf = [0; 4];
    while let Ok(()) = read_exact(&mut buf) {{
        let x = f32::from_bits(u32::from_ne_bytes(buf));
        let y = unsafe {{ {0}(x) }};
       stdout.write_all(&y.to_bits().to_ne_bytes())?;
    }}

    Ok(())
}}
", fun);
            let main = format!("{} {}", include_str!("templates/main_rt.rs"), run);

            File::create("math/src/main.rs")?.write_all(main.as_bytes())?;

            assert!(
                Command::new("cross")
                    .args(&["build", "--target", "thumbv7em-none-eabi", "--release"])
                    .current_dir("math")
                    .status()?
                .success()
            );

            let mut qemu = Command::new("qemu-arm")
                .arg("math/target/thumbv7em-none-eabi/release/math")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            qemu.stdin.as_mut().take().unwrap().write_all(F32)?;

            let output = qemu.wait_with_output()?;

            File::create(concat!("bin/output/newlib.", stringify!($fun)))?
                .write_all(&output.stdout)?;
        )+
    }
}

macro_rules! f32f32 {
    ($($fun:ident,)+) => {
        $(
            let fun = stringify!($fun);

            fs::create_dir_all("math/src")?;

            let run = format!("
fn run() -> Result<(), ()> {{
    #[link(name = \"m\")]
    extern \"C\" {{
        fn {0}(_: f32, _: f32) -> f32;
    }}
    let mut stdout = hio::hstdout()?;
    let mut chunk = [0; 8];
    while let Ok(()) = read_exact(&mut chunk) {{
        let mut buf = [0; 4];
        buf.copy_from_slice(&chunk[..4]);
        let x0 = f32::from_bits(u32::from_ne_bytes(buf));

        buf.copy_from_slice(&chunk[4..]);
        let x1 = f32::from_bits(u32::from_ne_bytes(buf));

        let y = unsafe {{ {0}(x0, x1) }};

        stdout.write_all(&y.to_bits().to_ne_bytes())?;
    }}

    Ok(())
}}
", fun);
let main = format!("{} {}", include_str!("templates/main_rt.rs"), run);

            File::create("math/src/main.rs")?.write_all(main.as_bytes())?;

            assert!(
                Command::new("cross")
                    .args(&["build", "--target", "thumbv7em-none-eabi", "--release"])
                    .current_dir("math")
                    .status()?
                .success()
            );

            let mut qemu = Command::new("qemu-arm")
                .arg("math/target/thumbv7em-none-eabi/release/math")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            qemu.stdin.as_mut().take().unwrap().write_all(F32)?;

            let output = qemu.wait_with_output()?;

            File::create(concat!("bin/output/newlib.", stringify!($fun)))?
                .write_all(&output.stdout)?;
        )+
    }
}

macro_rules! f32f32f32 {
    ($($fun:ident,)+) => {
        $(
            let fun = stringify!($fun);

            fs::create_dir_all("math/src")?;

            let run = format!("
fn run() -> Result<(), ()> {{
    #[link(name = \"m\")]
    extern \"C\" {{
        fn {0}(_: f32, _: f32, _: f32) -> f32;
    }}
    let mut stdout = hio::hstdout()?;
    let mut chunk = [0; 12];
    while let Ok(()) = read_exact(&mut chunk) {{
        let mut buf = [0; 4];
        buf.copy_from_slice(&chunk[..4]);
        let x0 = f32::from_bits(u32::from_ne_bytes(buf));

        buf.copy_from_slice(&chunk[4..8]);
        let x1 = f32::from_bits(u32::from_ne_bytes(buf));

        buf.copy_from_slice(&chunk[8..]);
        let x2 = f32::from_bits(u32::from_ne_bytes(buf));

        let y = unsafe {{ {0}(x0, x1, x2) }};

        stdout.write_all(&y.to_bits().to_ne_bytes())?;
    }}

    Ok(())
}}
", fun);
let main = format!("{} {}", include_str!("templates/main_rt.rs"), run);

            File::create("math/src/main.rs")?.write_all(main.as_bytes())?;

            assert!(
                Command::new("cross")
                    .args(&["build", "--target", "thumbv7em-none-eabi", "--release"])
                    .current_dir("math")
                    .status()?
                .success()
            );

            let mut qemu = Command::new("qemu-arm")
                .arg("math/target/thumbv7em-none-eabi/release/math")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            qemu.stdin.as_mut().take().unwrap().write_all(F32)?;

            let output = qemu.wait_with_output()?;

            File::create(concat!("bin/output/newlib.", stringify!($fun)))?
                .write_all(&output.stdout)?;
        )+
    }
}
