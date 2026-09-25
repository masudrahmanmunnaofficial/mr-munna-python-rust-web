//! Approximate NOAA solar prayer times. Check with local mosque for exact times.
use chrono::{Datelike, NaiveDate};
use std::f64::consts::PI;

pub const LOCATIONS:[(&str,f64,f64);8]=[("Dhaka",23.8103,90.4125),("Chattogram",22.3569,91.7832),("Sylhet",24.8949,91.8687),("Rajshahi",24.3745,88.6042),("Khulna",22.8456,89.5403),("Barishal",22.701,90.3535),("Rangpur",25.7439,89.2752),("Mymensingh",24.7471,90.4203)];
fn r(x:f64)->f64{x*PI/180.} fn d(x:f64)->f64{x*180./PI}
fn julian(mut y:i32,mut m:i32,day:i32)->f64{if m<=2{y-=1;m+=12} (365.25*(y+4716)as f64).floor()+(30.6001*(m+1)as f64).floor()+day as f64-1524.5}
fn hour(lat:f64,decl:f64,angle:f64)->Option<f64>{let den=r(lat).cos()*r(decl).cos();if den.abs()<1e-10{return None}let c=r(90.+angle).cos()/den-r(lat).tan()*r(decl).tan();if c>1.{None}else if c< -1.{Some(180.)}else{Some(d(c.acos()))}}
fn clock(v:Option<f64>)->String{v.map(|v|{let mins=(v.round()as i64).rem_euclid(1440);format!("{:02}:{:02}",mins/60,mins%60)}).unwrap_or_else(||"--:--".into())}
pub fn times(date:NaiveDate,lat:f64,lon:f64,karachi:bool,hanafi:bool)->[(String,String);6]{
let t=(julian(date.year(),date.month()as i32,date.day()as i32)+0.5-2451545.)/36525.;
let l=(280.46646+t*(36000.76983+t*0.0003032)).rem_euclid(360.);let m=357.52911+t*(35999.05029-t*0.0001537);
let e=0.016708634-t*(0.000042037+0.0000001267*t);
let c=r(m).sin()*(1.914602-t*(0.004817+0.000014*t))+r(2.*m).sin()*(0.019993-0.000101*t)+r(3.*m).sin()*0.000289;
let omega=125.04-1934.136*t;let lam=l+c-0.00569-0.00478*r(omega).sin();let eps=23.439291-0.0130042*t+0.00256*r(omega).cos();
let decl=d((r(eps).sin()*r(lam).sin()).asin());let y=r(eps/2.).tan().powi(2);
let eq=4.*d(y*r(2.*l).sin()-2.*e*r(m).sin()+4.*e*y*r(m).sin()*r(2.*l).cos()-0.5*y*y*r(4.*l).sin()-1.25*e*e*r(2.*m).sin());
let noon=720.-4.*lon-eq+360.;let rise=hour(lat,decl,0.833);let(fajr_angle,isha_angle)=if karachi{(18.,18.)}else{(18.,17.)};
let fajr=hour(lat,decl,fajr_angle);let isha=hour(lat,decl,isha_angle);let factor=if hanafi{2.}else{1.};
let asr_angle=d((1./(factor+r((lat-decl).abs()).tan())).atan());let asr=hour(lat,decl,-asr_angle);
let values=[fajr.map(|x|noon-x*4.),rise.map(|x|noon-x*4.),Some(noon),asr.map(|x|noon+x*4.),rise.map(|x|noon+x*4.),isha.map(|x|noon+x*4.)];
["Fajr","Sunrise","Dhuhr","Asr","Maghrib","Isha"].into_iter().zip(values.into_iter().map(clock)).map(|(n,v)|(n.into(),v)).collect::<Vec<_>>().try_into().unwrap()
}
#[cfg(test)]mod tests{use super::*;#[test]fn dhaka_order(){let d=NaiveDate::from_ymd_opt(2026,9,25).unwrap();let t=times(d,23.8103,90.4125,true,true);assert!(t[0].1<t[1].1&&t[1].1<t[2].1&&t[2].1<t[3].1&&t[3].1<t[4].1&&t[4].1<t[5].1)} }
