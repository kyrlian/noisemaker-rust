//https://docs.rs/cpal/0.8.2/cpal/index.html
mod signal;
mod oscillator;
mod sampler;
mod track;

fn main() {
    println!("MAKE SOME NOISE!");

    //init sound
    use oscillator::{Oscillator, ShapeKind};
    use track::Track;
    use std::sync::Arc;

    let env_shape = vec!((0.0,0.0), (0.1,0.8), (0.2,0.5), (0.4,0.0), (0.5,0.0));//Vector of x,y tuples
    let env_signal = Oscillator::new(1.0,0.0,1.0,ShapeKind::CUSTOM, None, None, None, Some(env_shape));
    let osc = Oscillator::new(440.0,0.0,1.0,ShapeKind::SIN, None, None, Some(Arc::new(env_signal)), None);
    let osc2 = osc.clone();

    //let no_track_10 = [None,None,None,None,None,None,None,None,None,None];
    //let no_osc_10 = [None,None,None,None,None,None,None,None,None,None];
    let track1 = Track::new(vec!((Arc::new(osc),0.0,1.0)),1.0,None);
    let track2 = Track::new(vec!((Arc::new(track1),0.0,1.0)),1.0,None);

    //run sampler
    use sampler::Sampler;
    println!("Playing first sample");
    let thread1 = Sampler::new(Arc::new(track2));
    println!("Playing second sample");
    //let osc2 = Oscillator::new(220.0,0.0,1.0,ShapeKind::SIN, None, None, None, None);
    let thread2 = Sampler::new(Arc::new(osc2));

    //Wait for threads
    thread1.thread_handle.join().expect("thread1 failed");
    thread2.thread_handle.join().expect("thread2 failed");

}