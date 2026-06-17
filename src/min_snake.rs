use std::{io,io::Write,error::*,time::*,collections::*};
use crossterm::{*,event::{*,KeyCode::*},cursor::*,style::*,terminal::*};
use rand::*;

pub fn min_snake()->Result<(),Box<dyn Error>>{
  let (w, h)=size()?;
  let (c_x, c_y)=(w as i16/2,h as i16/2);
  let mut o=io::stdout();
  let mut d: VecDeque<(i16,i16)>=VecDeque::new();
  let mut v=(1, 0);
  let mut a=(random_range(1..w-1),random_range(1..h-1));
  let mut s='>';
  let mut m=200;

  enable_raw_mode()?;
  queue!(o,EnterAlternateScreen,Hide)?;
  for x in 0..w{queue!(o,MoveTo(x,0),Print('+'),MoveTo(x,h-1),Print('+'))?;}
  for y in 0..h{queue!(o,MoveTo(0,y),Print('+'),MoveTo(w-1,y),Print('+'))?;}
  queue!(o,MoveTo(a.0, a.1),Print('O'))?;
  for n in 0..3{
    d.push_back((c_x+n,c_y));
    queue!(o,MoveTo((c_x+n) as u16,c_y as u16),Print('#'))?;
  }
  o.flush()?;

  loop {
    if poll(Duration::from_millis(m))?&&let Event::Key(event)=read()?{
        match event.code{
      Char('q')=>break,
      Char('w') if s!='v'=>(s,v)=('^',(0,-1)),
      Char('a') if s!='>'=>(s,v)=('<',(-1,0)),
      Char('s') if s!='^'=>(s,v)=('v',(0,1)),
      Char('d') if s!='<'=>(s,v)=('>',(1,0)),
      _=>{}
    }}

    if let Some((hx,hy))=d.pop_back(){
      if hx<=0||hx>=w as i16-1||hy<=0||hy>=h as i16-1||d.contains(&(hx,hy)){break;}
      d.push_back((hx,hy));
    }

    if let Some(&(hx,hy))=d.back() && hx==a.0 as i16 && hy==a.1 as i16
        && let Some(&(x,y))=d.front(){
      d.push_front((x,y));
      loop{
        a=(random_range(1..w-1),random_range(1..h-1));
        if !d.contains(&(a.0 as i16,a.1 as i16)){break}}
      execute!(o,MoveTo(a.0, a.1),Print('O'))?;
      m=((m as f32*0.90) as u64).max(50);
    }

    if let Some((x,y))=d.pop_front(){
      execute!(o,MoveTo(x as u16, y as u16),Print(' '))?;
    }
    if let Some(&(x,y))=d.back(){
      execute!(o,MoveTo(x as u16, y as u16),Print('#'))?;
      let n = ((x+v.0),(y+v.1));
      execute!(o,MoveTo(n.0 as u16, n.1 as u16),Print(s))?;
      d.push_back(n);
    }
  }

  execute!(o,LeaveAlternateScreen,Show)?;
  disable_raw_mode()?;
  println!("Score: {}",d.len()-2);
  Ok(())
}