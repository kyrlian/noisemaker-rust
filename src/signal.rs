pub trait Signal{
    fn get_val(&self,t:f32) -> f32;
    fn bind(&self,v:f32) -> f32{
        if v < -1.0{
            println!("Norm min");
            -1.0
        }else if v > 1.0{
            println!("Norm max");
            1.0
        }else{
            v
        }
    }
    fn get_norm_val(&self,t:f32) -> f32{
        self.bind(self.get_val(t))
    }
}