use std::io;
use tts::*;

fn main() {
    println!("Hello, world!");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "quit" {
                    break;
                } else {
                    let result = convert_text_to_audio(input);
                    match result {
                        Ok(_) => println!("Success"),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
fn convert_text_to_audio(text: String) -> Result<String , Error> {
    let mut tts = Tts::default()?;
    if Tts::screen_reader_available() {
        println!("A screen reader is available on this platform.");
    } else {
        println!("No screen reader is available on this platform.");
    }
    let Features {
        utterance_callbacks,
        ..
    } = tts.supported_features();
    if utterance_callbacks {
        tts.on_utterance_begin(Some(Box::new(|utterance| {
            println!("Started speaking {:?}", utterance)
        })))?;
        tts.on_utterance_end(Some(Box::new(|utterance| {
            println!("Finished speaking {:?}", utterance)
        })))?;
        tts.on_utterance_stop(Some(Box::new(|utterance| {
            println!("Stopped speaking {:?}", utterance)
        })))?;
    }
    let Features { is_speaking, .. } = tts.supported_features();
    if is_speaking {
        println!("Are we speaking? {}", tts.is_speaking()?);
    }
    tts.speak(text, false)?;
 
    // loop until speak is finished
    while tts.is_speaking()? {
        // when speaking is finished, we return the result
    }
    return Ok("Everything goes well".to_string());
}
