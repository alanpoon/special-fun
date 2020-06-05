use wasm_bindgen::prelude::*;
use core::ops::{Add, Sub};
macro_rules! inExtern{
  ( $s_alias:ident,$(($r_alias:ident,$r_type:ty))|*$(|)*,$o_type:ty,$t_alias:ident
     ) =>{
      #[wasm_bindgen]
      extern "C"{
        #[wasm_bindgen(js_namespace=cephes)]
        pub fn $s_alias($($r_alias:$r_type,)*) -> $o_type;
      }
     }
}

macro_rules! implFn{
  ( $s_alias:ident,($r_alias:ident,$r_type:ty)|$(($r_alias2:ident,$r_type2:ty))|*$(|)*,$o_type:ty,$e_alias:ident  
    ) =>{
    fn $e_alias(self,$($r_alias2:Self,)*) -> Self{
      $s_alias(self as f64,$($r_alias2 as f64,)*)  as Self
    }
  };
  ( $s_alias:ident,($r_alias:ident,$r_type:ty),$o_type:ty,$e_alias:ident  
  ) =>{
    fn $e_alias(self) -> Self{
      $s_alias(self as f64) as Self
    }
  }   
}

inExtern!(_cephes_fac,(i,f64),f64,fac);
inExtern!(_cephes_lgam,(i,f64),f64,loggamma);
inExtern!(_cephes_ndtr,(i,f64),f64,norm);
inExtern!(_cephes_ndtri,(i,f64),f64,norm_inv);
inExtern!(_cephes_beta,(a,f64)|(b,f64),f64,beta);
inExtern!(_cephes_incbet,(a,f64)|(b,f64)|(c,f64),f64,betainc);
inExtern!(_cephes_psi,(i,f64),f64,digamma);
inExtern!(_cephes_igam,(a,f64)|(b,f64),f64,gammainc);
inExtern!(_cephes_gamma,(a,f64),f64,gamma);
inExtern!(_cephes_zetac,(a,f64),f64,riemann_zeta);
inExtern!(_cephes_erfc,(a,f64),f64,erfc);
inExtern!(_cephes_hyp2f1,(a,f64)|(b,f64)|(c,f64)|(d,f64),f64,hyp2f1);

pub trait FloatSpecial: Copy + Add<Output=Self> + Sub<Output=Self> {
  fn fac(self)->Self;
  fn loggamma(self)->Self;
  fn norm(self)->Self;
  fn norm_inv(self)->Self;
  fn beta(self,b: Self)->Self;
  fn betainc(self,a:Self,b:Self)->Self;
  fn digamma(self)->Self;
  fn gamma(self)->Self;
  fn erfc(self)->Self;
  fn hyp2f1(self,a: Self,b: Self, c: Self)->Self;
  fn riemann_zeta(self)->Self;
  fn gammainc(self, a: Self)->Self;
  fn logbeta(self, b:Self)->Self;
}
impl FloatSpecial for f64 {
  implFn!(_cephes_fac,(i,f64),f64,fac);
  implFn!(_cephes_lgam,(i,f64),f64,loggamma);
  implFn!(_cephes_ndtr,(i,f64),f64,norm);
  implFn!(_cephes_ndtri,(i,f64),f64,norm_inv);
  implFn!(_cephes_beta,(a,f64)|(b,f64),f64,beta);
  fn betainc(self, a: f64, b: f64) -> f64 {
    _cephes_incbet(a, b, self) 
  }
  implFn!(_cephes_psi,(i,f64),f64,digamma);
  implFn!(_cephes_gamma,(a,f64),f64,gamma);
  implFn!(_cephes_erfc,(a,f64),f64,erfc);
  fn hyp2f1(self, a: f64, b: f64, c: f64)->f64{
    _cephes_hyp2f1(a,b,c,self)
  }
  fn riemann_zeta(self) -> f64 {
    1.0 + _cephes_zetac(self)
  }
  fn gammainc(self,a: Self) ->Self{
    _cephes_igam(a,self)
  }
  fn logbeta(self, b: Self) -> Self {
      self.loggamma() + b.loggamma() - (self + b).loggamma()
  }
}
impl FloatSpecial for f32 {
  implFn!(_cephes_fac,(i,f64),f64,fac);
  implFn!(_cephes_lgam,(i,f64),f64,loggamma);
  implFn!(_cephes_ndtr,(i,f64),f64,norm);
  implFn!(_cephes_ndtri,(i,f64),f64,norm_inv);
  implFn!(_cephes_beta,(a,f64)|(b,f64),f64,beta);
  fn betainc(self, a: f32, b: f32) -> f32 {
    _cephes_incbet(a as f64, b as f64, self as f64) as f32
  }
  implFn!(_cephes_psi,(i,f64),f64,digamma);
  implFn!(_cephes_gamma,(a,f64),f64,gamma);
  implFn!(_cephes_erfc,(a,f64),f64,erfc);
  fn hyp2f1(self, a: f32, b: f32, c: f32)->Self{
    _cephes_hyp2f1(a as f64,b as f64,c as f64,self as f64)as f32
  }
  fn riemann_zeta(self) -> Self {
    1.0 + _cephes_zetac(self as f64)as f32
  }
  fn gammainc(self,a: Self) ->Self{
    _cephes_igam(a as f64,self as f64)as f32
  }
  fn logbeta(self, b: Self) -> Self {
      (self.loggamma() + b.loggamma() - (self + b).loggamma()) as f32
  }
}