mod audio_rodio;
mod blocks;
mod tetris;

use audio_rodio::AudioManager;
use tetris::Tetris;

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;

    let mut audio_manager = AudioManager::new();
    audio_manager.init()?;

    // Tetris 게임 시작
    let mut game = Tetris::new(&sdl_context, audio_manager)?;
    game.run()?;

    Ok(())
}
