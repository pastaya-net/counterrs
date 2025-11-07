use crate::TimeoutFuture;
use crate::audio::*;
use crate::bootstrap_image;

/// Basic macro for playing audio
/// Use like `audio!(AUDIO_69)`
/// probably don't pass in something that doesn't have the `.with()` method
macro_rules! audio {
    ($audio:expr) => {
        $audio.with(|a| {
            let _ = a.play();
        })
    };
}

/// Plays a set of special effects on a certain "number"  
/// All numbers that have a special effect:  
/// - 1
/// - 21
/// - 42
/// - 67
/// - 69
/// - 420
/// - 666
/// - 777
/// - 1000
/// - 12345
/// - 9000
/// - 80085

// TODO: pls make it configurable at runtime
pub async fn special_effects(count: u32) {
    // WARN: this code is made in yandere dev style
    match count {
        1 => {
            audio!(AUDIO_1_1);
            TimeoutFuture::new(2000).await;
            AUDIO_1_2.with(|a| {
                let _ = a.play();
            });
        }

        21 => audio!(AUDIO_21),

        42 => audio!(AUDIO_42),

        67 => {
            audio!(AUDIO_67);
            let _ = bootstrap_image(
                "https://i.kym-cdn.com/photos/images/newsfeed/003/128/463/b28",
                Some("the 67 kid with the blue lasers coming out of his eyes".to_string()),
                3000,
                Some("half".to_string()),
            );
        }

        69 => audio!(AUDIO_69),

        420 => audio!(AUDIO_420),

        666 => audio!(AUDIO_666),

        777 => {
            let _ = bootstrap_image(
                "https://media1.tenor.com/m/WUWygJ0Fwz8AAAAd/jago33-slot-machine.gif",
                Some("slot machine go BRRRRRRRRRRRR".to_string()),
                6000,
                Some("slotmachine".to_string()),
            );

            audio!(AUDIO_777);
        }

        1000 => audio!(AUDIO_1000), // TODO: add xbox gif at fixed bottom

        12345 => audio!(AUDIO_12345),

        9000 => audio!(AUDIO_9000),

        80085 => audio!(AUDIO_80085),

        _ => {}
    }
}
