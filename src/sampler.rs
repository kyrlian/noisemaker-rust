
//use crate::oscillator::Oscillator;
use crate::signal::Signal;
use std::thread;
use cpal::{EventLoop, StreamId, StreamData, UnknownTypeOutputBuffer};
use std::sync::Arc;

//the sampler object to keep track of time
pub struct Sampler{
    pub thread_handle: thread::JoinHandle<i32>,
}

impl Sampler{
    pub fn new(signal:Arc<Signal + Sync + Send>) -> Sampler{
        //Spawn new thread that will hold the sampler
        let th : thread::JoinHandle<i32> = thread::spawn(move || {//|| is an anonymous closure that will run inside the thread
            //Setup device
            let device = cpal::default_output_device().expect("no output device available");
            //choose format
            let mut supported_formats_range = device.supported_output_formats().expect("error while querying formats");
            let format = supported_formats_range.next().expect("no supported format?!").with_max_sample_rate();
            let sample_rate  = format.sample_rate.0;
            let sample_time = 1.0 / (sample_rate as f32);
            let mut time = 0.0;
            //init and start the stream
            let event_loop = EventLoop::new();
            let stream_id = event_loop.build_output_stream(&device, &format).expect("error creating output stream");
            event_loop.play_stream(stream_id);
            event_loop.run(move | _stream_id: StreamId, stream_data: StreamData | {//we create an anonymous closure that fills the buffer
                match stream_data {
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {//The value 0 corresponds to 32768.
                        println!("U16");
                        for elem in buffer.iter_mut() {
                            *elem = u16::max_value() / 2;
                        }
                    },
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {//The value 0 corresponds to 0.
                        println!("I16");
                        for elem in buffer.iter_mut() {
                            *elem = 0;
                        }
                    },
                    StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {//The boundaries are (-1.0, 1.0).
                        //println!("F32");
                        for elem in buffer.iter_mut() {
                            time += sample_time;
                            let v= signal.get_norm_val(time);//the signal is passed here as part of the context
                            //println!("{}",v);
                            *elem =  v;
                        }
                    },
                    _ => (),
                }
            })
        });
        //thread is running
        //save the thread handle as object element
        Sampler {
            thread_handle: th,
        }
    }
}