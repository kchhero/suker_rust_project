mod audio;
mod blocks;
mod tetris;

use audio::AudioManager;
use tetris::Tetris;

fn main() -> Result<(), String> {
    // SDL2 전체 초기화
    let sdl_context = sdl2::init()?;

    // 오디오 서브시스템 초기화 및 배경음 설정
    let audio_subsystem = sdl_context.audio()?;
    let mut audio_manager = AudioManager::new();
    audio_manager.init(&audio_subsystem)?;

    // Tetris 게임 인스턴스 실행
    let mut game = Tetris::new(&sdl_context, audio_manager)?;
    game.run()?;

    Ok(())
}
