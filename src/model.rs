use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry { pub id: String, pub kind: String, pub title: String, pub body: String, pub date: String, pub mood: String, pub tags: String, pub place: String, pub done: bool }

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AppData { pub entries: Vec<Entry>, pub sheets: Vec<Sheet>, pub birthdate: String, pub display_name: String, pub latitude: f64, pub longitude: f64 }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sheet { pub id: String, pub title: String, pub cells: Vec<String> }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page { Dashboard, Journal, Calendar, Timeline, Gallery, Activities, Thoughts, Goals, Nutrition, Profile, Body, Calculas, Prayer, Bangladesh, Backup }
impl Page {
 pub fn all() -> &'static [(Page, &'static str, &'static str)] { use Page::*; &[(Dashboard,"Dashboard","◈"),(Journal,"Journal","✎"),(Calendar,"Calendar","▦"),(Timeline,"Timeline","◷"),(Gallery,"Life gallery","▤"),(Activities,"Activities","⚡"),(Thoughts,"Thoughts","✧"),(Goals,"Goals","◎"),(Nutrition,"Nutrition","◉"),(Profile,"Profile","♙"),(Body,"BMI & BMR","♡"),(Calculas,"Calculas","∑"),(Prayer,"Clock & prayer","◴"),(Bangladesh,"Bangladesh","◇"),(Backup,"Backup & settings","⚙")] }
 pub fn title(self)-> &'static str { Self::all().iter().find(|(p,_,_)|*p==self).map(|(_,t,_)|*t).unwrap_or("Dashboard") }
 pub fn slug(self)-> &'static str { match self { Self::Dashboard=>"dashboard",Self::Journal=>"journal",Self::Calendar=>"calendar",Self::Timeline=>"timeline",Self::Gallery=>"gallery",Self::Activities=>"activities",Self::Thoughts=>"thoughts",Self::Goals=>"goals",Self::Nutrition=>"nutrition",Self::Profile=>"profile",Self::Body=>"body",Self::Calculas=>"calculas",Self::Prayer=>"prayer",Self::Bangladesh=>"bangladesh",Self::Backup=>"backup" } }
 pub fn from_slug(s:&str)->Self { Self::all().iter().map(|(p,_,_)|*p).find(|p|p.slug()==s).unwrap_or(Self::Dashboard) }
}

#[cfg(test)] mod tests { use super::*; #[test] fn routes(){ assert_eq!(Page::all().len(),15); assert!(!Page::all().iter().any(|(_,t,_)|t.contains("Sexual")||t.contains("Vault"))); assert_eq!(Page::from_slug("prayer"),Page::Prayer); } }
