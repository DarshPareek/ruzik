use macroquad::{
    audio::{self, PlaySoundParams, play_sound},
    prelude::*,
};

#[macroquad::main("BasicShapes")]
async fn main() {
    let music = audio::load_sound("/home/darsh/Music/output.wav")
        .await
        .unwrap();
    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 1.,
        },
    );
    let mut toggle: bool = false;
    loop {
        if is_key_down(KeyCode::Space) {
            toggle = !toggle;
        }
        if toggle {
            audio::stop_sound(&music);
        } else {
            play_sound(
                &music,
                PlaySoundParams {
                    looped: true,
                    volume: 1.,
                },
            );
        }
        clear_background(RED);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
