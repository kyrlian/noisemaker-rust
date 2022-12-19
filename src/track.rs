
use crate::signal::Signal;
use crate::oscillator::Oscillator;
use std::sync::Arc;

#[derive(Clone)]
pub struct Track<T: Signal>{
    //signal_list: [Option<(Arc<T>,f32,f32)>;10],
    signal_list: Vec<(Arc<T>,f32,f32)>,

    //track_list: [Option<(Arc<Track>,f32,f32)>;10],
    //osc_list: [Option<(Arc<Oscillator>,f32,f32)>;10],
    ampl: f32,
    lfo_ampl: Option<Arc<Oscillator>>,
}

impl<T: Signal> Track<T>{
    //pub fn new(tl:[Option<(Arc<Track>,f32,f32)>;10],ol:[Option<(Arc<Oscillator>,f32,f32)>;10],a:f32,lfoa:Option<Arc<Oscillator>>) -> Track{
    pub fn new(sl:Vec<(Arc<T>,f32,f32)>,a:f32,lfoa:Option<Arc<Oscillator>>) -> Track<T>{
        Track{
            signal_list: sl,
            //track_list: tl,
            //osc_list: ol,
            ampl: a,
            lfo_ampl: lfoa,
        }
    }
    fn get_ampl(&self,t:f32) -> f32 {
        self.ampl + match &self.lfo_ampl {Some(o) => o.get_val(t), None => 0.0,}
    }
}

impl<T: Signal>  Signal for Track<T>{
    fn get_val(&self, t : f32) -> f32 {
        //fn add_list_elems(l:&[Option<(Arc<impl Signal>,f32,f32)>;10],t:f32) -> f32{
        let mut r = 0.0;
        for tuple in self.signal_list.iter(){
            match tuple{
                (signal,from,to)=>
                    if(t >= *from) & (t < *to){//this signal is active
                        let inner_t = t - from;
                        r += signal.get_val(inner_t);
                    }
                _ => {}
            }
        }
        //}
        //(add_list_elems(&self.osc_list,t) + add_list_elems(&self.track_list,t) ) * self.get_ampl(t)
        r * self.get_ampl(t)
    }
}
