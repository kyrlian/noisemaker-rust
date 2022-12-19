#[derive(Copy, Clone)]
pub enum ShapeKind{
    SIN,
    SQR,
    FLAT,
    CUSTOM,
}

#[derive(Clone)]
pub struct Oscillator{
    freq: f32,
    phase: f32,
    ampl: f32,
    shape: ShapeKind,
    lfo_freq: Option<Arc<Oscillator>>,
    lfo_phase: Option<Arc<Oscillator>>,
    lfo_ampl: Option<Arc<Oscillator>>,
    //custom_shape: Option<[(f32,f32);5]>,
    custom_shape: Option<Vec<(f32,f32)>>,
}

use std::sync::Arc;

impl Oscillator{
    pub fn new(f:f32,p:f32,a:f32,s:ShapeKind,lfof:Option<Arc<Oscillator>>,lfop:Option<Arc<Oscillator>>,lfoa:Option<Arc<Oscillator>>,cs:Option<Vec<(f32,f32)>>) -> Oscillator {
        Oscillator{
            freq: f,
            phase: p,
            ampl: a,
            shape: s,
            lfo_freq: lfof,
            lfo_phase: lfop,
            lfo_ampl: lfoa,
            custom_shape :cs,
        }
    }

    //fn compute(&self,x:f32,a:&[(f32,f32);5])->f32{
    fn compute_custom_val(&self,x:f32,a:&Vec<(f32,f32)>)->f32{
        let lastx:f32 = a.last().expect("Empty array").0;
        let mut previousx:f32 = a.first().expect("Empty array").0;
        let mut previousy:f32 = a.first().expect("Empty array").1;
        let xmod:f32 = x % lastx;
        for (ax,ay) in a.iter(){
            if xmod < *ax {
                let nextx = *ax;
                let nexty = *ay;
                let r = (xmod - previousx) / (nextx - previousx) * (nexty - previousy);
                return r
            }
            previousx = *ax;
            previousy = *ay;
        }
        0.0
    }
    fn get_custom_val(&self,x:f32) -> f32{
        match &self.custom_shape {
            Some(v) => self.compute_custom_val(x,v),
            None => 0.0,
        }
    }
    fn get_freq(&self,t:f32) -> f32 {
        self.freq + match &self.lfo_freq {Some(o) => o.get_val(t), None => 0.0,}
    }
    fn get_phase(&self,t:f32) -> f32 {
        self.phase + match &self.lfo_phase {Some(o) => o.get_val(t), None => 0.0,}
    }
    fn get_ampl(&self,t:f32) -> f32 {
        self.ampl + match &self.lfo_ampl {Some(o) => o.get_val(t), None => 0.0,}
    }
}

use crate::signal::Signal;
impl Signal for Oscillator {
    fn get_val(&self, t : f32) -> f32 {
        let p:f32 = 1.0/self.get_freq(t);//period
        let x:f32 = t * self.get_freq(t) + self.get_phase(t);
        let y:f32 = match self.shape{
            ShapeKind::SIN => x.sin(),
            ShapeKind::FLAT => 1.0,
            ShapeKind::SQR => if x < p/2.0 {0.0}else{1.0},
            ShapeKind::CUSTOM => self.get_custom_val(x),
            //_ => 0.0,
        };
        y * self.get_ampl(t)
    }
}
