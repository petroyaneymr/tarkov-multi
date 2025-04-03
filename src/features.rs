use rand::Rng;

pub async fn load_features() {
    let aimbot_enabled = true;
    let esp_enabled = true;
    if aimbot_enabled {
        activate_aimbot().await;
    }
    if esp_enabled {
        activate_esp().await;
    }
}

async fn activate_aimbot() {
    loop {
        let target = find_target();
        if let Some(target) = target {
            aim_at_target(target).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

async fn activate_esp() {
    loop {
        display_esp();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

fn find_target() -> Option<(f32, f32)> {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Some((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)))
    } else {
        None
    }
}

async fn aim_at_target(target: (f32, f32)) {
    Command::new("aimbot_command")
        .arg(format!("{} {}", target.0, target.1))
        .output()
        .expect("Failed to execute aimbot command");
}

fn display_esp() {
    println!("ESP: Displaying targets");
}