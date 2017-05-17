
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
use chrono::prelude::*;

fn main() {
    // let now = chrono::UTC::now();
    let now = Local::now();
    // source: http://chris.com/ascii/index.php?art=events/birthday
    let ascii_art = r#"
           ~                  ~
     *                   *                *       *
                  *               *
  ~       *                *         ~    *          
              *       ~        *              *   ~
                  )         (         )              *
    *    ~     ) (_)   (   (_)   )   (_) (  *
           *  (_) # ) (_) ) # ( (_) ( # (_)       *
              _#.-#(_)-#-(_)#(_)-#-(_)#-.#_    
  *         .' #  # #  #  # # #  #  # #  # `.   ~     *
           :   #    #  #  #   #  #  #    #   :   
    ~      :.       #     #   #     #       .:      *
        *  | `-.__                     __.-' | *
           |      `````"""""""""""`````      |         *
     *     |         | ||\ |~)|~)\ /         |    
           |         |~||~\|~ |~  |          |       ~
   ~   *   |                                 | * 
           |      |~)||~)~|~| ||~\|\ \ /     |         *
   *    _.-|      |~)||~\ | |~|| /|~\ |      |-._  
      .'   '.      ~            ~           .'   `.  *
      :      `-.__                     __.-'      :
       `.         `````"""""""""""`````         .'
         `-.._                             _..-'
              `````""""-----------""""`````    
    
                                                               tttt
                                                             ttt:::t
                                                             t:::::t
                                                             t:::::t
rrrrr   rrrrrrrrr   uuuuuu    uuuuuu      ssssssssss   ttttttt:::::ttttttt
r::::rrr:::::::::r  u::::u    u::::u    ss::::::::::s  t:::::::::::::::::t
r:::::::::::::::::r u::::u    u::::u  ss:::::::::::::s t:::::::::::::::::t
rr::::::rrrrr::::::ru::::u    u::::u  s::::::ssss:::::stttttt:::::::tttttt
 r:::::r     r:::::ru::::u    u::::u   s:::::s  ssssss       t:::::t
 r:::::r     rrrrrrru::::u    u::::u     s::::::s            t:::::t
 r:::::r            u::::u    u::::u        s::::::s         t:::::t
 r:::::r            u:::::uuuu:::::u  ssssss   s:::::s       t:::::t    tttttt
 r:::::r            u:::::::::::::::uus:::::ssss::::::s      t::::::tttt:::::t
 r:::::r             u:::::::::::::::us::::::::::::::s       tt::::::::::::::t
 r:::::r              uu::::::::uu:::u s:::::::::::ss          tt:::::::::::tt
 rrrrrrr                uuuuuuuu  uuuu  sssssssssss              ttttttttttt"#;

    // if "May 15" == now.format("%b %-d").to_string() {
    if now.month() == 5 && (now.day() >= 15 && now.day() < 22)  {
        println!("{}", ascii_art);
    } else {
        println!(":( come back on the 15th of May");
    }
    
    

    
}
