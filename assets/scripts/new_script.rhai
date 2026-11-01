fn update() {
    let speed = 300.0;
    if left  { vel_x = -speed; }
    if right { vel_x =  speed; }
    if !left && !right { vel_x = 0.0; }

    if up && grounded {
        vel_y = -500.0;
        grounded = false;
    }
}
