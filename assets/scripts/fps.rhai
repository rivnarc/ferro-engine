fn update() {
    if time - last_log >= 1.0 {
        log("FPS: " + fps);
        last_log = time;
    }
}
