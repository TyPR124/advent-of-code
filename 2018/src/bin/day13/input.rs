pub const INPUT: &str = r#"                                 /-----------------------------------------------\                                 /-------------------\              
  /------------------------------+-------------------------------------------\   |/--------------------------------+---------------\   |              
  |                              |                                           |   ||                    /-----------+---------------+---+-------------\
  |      /-----------------------+-------------------------------------------+---++--------------------+----------\|               |   |             |
  |   /--+-----------------------+-------------------\                       |   ||                    |          ||               |   |             |
  |   |  |                       |       /-----------+-----------------------+---++--------------------+----------++---------------+---+------\      |
  |   |  |   /-------------------+-------+-----------+-----------------------+---++--------------------+-----\    ||               |   |      |      |
  |   |  |   |                   |       |           |                       |   ||                    |     |    ||               |   |      |      |
  |   |  |   |                   |       |           |        /--------------+---++-----------------\  |     |    ||               |   |      |      |
  |   |  |   |                   |       |           |        |              |   ||                 |  |     |    ||               |   |      |      |
  |   |  |   |   /---------------+---\/--+-----------+--------+--------------+---++-----------------+--+-----+----++---------------+-\ |      |      |
  |   |  |   |   | /-------------+---++--+-----------+--------+--------------+---++\                |  |     |    ||               | | |      |      |
/-+---+--+---+---+-+-------------+---++--+-----------+--------+--------------+---+++--\             |  |     |    ||               | | |      |      |
|/+---+--+---+--\| |           /-+---++--+-----------+--------+--------\     |   |||  |             |  |     |    ||               | | |      |      |
|||   |  |   |  || |        /--+-+---++--+-----------+-----\  |/-------+-----+---+++--+-------------+--+-----+----++---------------+-+-+--\   |      |
||| /-+--+---+--++-+--------+--+-+---++--+-----------+-----+--++-------+-----+--\|||  |           /-+--+-----+----++---------------+-+-+--+---+----\ |
||| | |  |   |  || |  /-----+--+\|   ||  |           |     |  ||       |     |  ||||  |           | |  |     |    ||               | | |  |   |    | |
||| | |  |/--+--++-+--+--\  |  |||   ||  \-----------+-----+--++-------+-----+--++++--+-----------+-+--+-----+----++---------------+-+-+--+---/    | |
||| | |  ||  |  || |/-+--+--+--+++---++--------------+-----+--++-------+-----+--++++--+-----------+-+--+-----+----++--------\      | | |  |        | |
||| | |  ||  |  || || |  | /+--+++---++-------\      |/----+--++-------+-----+--++++--+-----------+-+--+-----+----++--------+------+\| |  |        | |
||| | |  ||  |  || || |  | ||  |||   ||       |      ||    |  || /-----+-----+--++++--+-----------+-+--+-----+----++--\     |      ||| |  |        | |
||| | |  ||  |  || || |  | ||  |||   ||      /+------++----+--++-+--\  |     |  ||||  |           | |  |     |    ||  |     |      ||| |  |        | |
|||/+-+--++--+--++-++-+--+-++--+++---++\     ||      ||    |  || |  |  |     |  ||||  |           | v  |     |    ||  |     |      ||| |  |        | |
||||| |  ||  |  || || |  | ||  |||   |||     ||      ||    |  || |  |/-+-----+--++++--+-----------+-+--+-----+----++--+-----+------+++-+--+---\    | |
|\+++-+--++--+--/| || |  | ||  |||   |||     ||      ||    |  || |  || |     |/-++++--+-----------+-+--+-----+----++--+----\|      ||| |  |   |    | |
\-+++-+--++--+---+-++-+--+-++--+++---+++-----++------++----+--++-+--++-+-----++-++++--/           | |  |     |    ||  |    ||      ||| |  |   |    | |
  ||| |  ||  |   | || |  | ||  |||   |||     ||      ||    |  ||/+--++-+-----++-++++--------------+-+--+-\   |    ||  |    ||      ||| |  |   |    | |
  ||| |  ||  |   |/++-+--+-++--+++---+++-----++------++----+--++++--++-+-----++-++++--------------+\|/-+-+---+----++--+----++------+++-+--+---+--\ | |
  ||| |  ||  |   |||| |  | ||  |||   |||     ||      ||/---+--++++--++-+-----++-++++--------------++++-+-+---+----++--+----++------+++-+\ |   |  | | |
  \++-+--++--+---++++-+--+-++--+++---+++-----++------+++---+--++++--++-+-----/| ||||              |||| | |   |    ||  |    ||      ||| || |   |  | | |
   || |  ||  |   |||| |  | || /+++---+++-----++------+++---+--++++--++-+------+-++++--------------++++-+-+---+----++--+----++------+++\|| |   |  | | |
   || |  ||  |   |||| \--+-++-++/|   |||     ||      |||   |  ||||  || |      | ||||              |||| | |  /+----++--+----++--\   |||||| |   |  | | |
   || |  ||  |   ||||    | || || |/--+++-----++------+++--\|  ||||  || |/-----+-++++--------------++++-+-+\ ||    ||  |    ||  |   |||||| |   |  | | |
  /++-+--++\ |   ||||  /-+-++-++-++--+++-----++------+++--++--++++--++-++-----+-++++-------------\|||| | || ||    ||  |    ||  |   |||||| |   |  | | |
  ||| |  ||| |   ||||  | | || || ||  |||   /-++------+++--++--++++--++-++-----+-++++-------------+++++-+-++-++-\  ||  |    ||  |   |||||| |   |  | | |
  ||| |  ||| |   ||||  | | || || || /+++---+-++------+++--++--++++--++-++-----+-++++--\          ||||| | || || |  ||  |    ||  |   |||||| |   |  | v |
/-+++-+--+++-+---++++--+-+-++-++-++-++++---+-++------+++\ ||  |\++--++-++-----+-++++--+----------+++++-+-++-++-+--++--+----++--+---++++++-/   |  | | |
| ||| |  ||| |   ||||  | | || || || ||||   | ||      |||| ||  | ||  || ||     | ||||  |          ||||| | || || |  ||  |    ||  |   ||||||     |  | | |
| ||| |  ||| |   \+++--+-+-++-++-++-+/||   | ||      |||| ||/-+-++--++-++-----+-++++--+----------+++++-+-++\|| |  ||  |    ||  |   ||||||     |  | | |
| ||| |  ||| |    |||  | | || || || | ||   | ||      |||| ||| | ||  || || /---+-++++--+----------+++++-+-+++++-+--++--+----++--+---++++++---\ |  | | |
| ||| |  ||| |    |||  | | || || || | ||/--+-++------++++-+++-+-++--++-++-+---+-++++--+----------+++++-+-+++++\|  ||  |    ||  |   ||||||   | |  | | |
| ||| | /+++-+----+++--+-+-++-++-++-+-+++--+-++--\   |||| ||| | ||  || || |   | ||||  |          ||||| | |||||||  ||  | /--++--+---++++++---+-+\ | | |
| ||| | |||| |    |||  | | |\-++-++-+-+++--+-++--+---++++-+/| | ||  || || |   | ||||  |          ||||| | |||||||  ||  | |  ||  |   ||||||   | || | | |
| ||| | |||| |/---+++--+-+-+--++-++-+-+++--+-++--+---++++-+-+-+\|\--++-++-+---+-++++--+----------+++++-+-+++++++--++--/ |  ||/-+---++++++---+\|| | | |
| ||| | |||| ||   |||  |/+-+--++-++-+-+++--+-++--+---++++-+-+-+++---++\|| |   | ||||  |    /-----+++++-+-+++++++--++----+\ ||| |   ||||||   |||| | | |
| ||| | |||| ||   |||  ||| |  || || | |||  | ||  |   |||| | | |||   ||||| |   | ||||  |    |     ||||| | |||||||  ||    || ||| |   ||||||   |||| | | |
| ||| | |||| ||   |||  ||| |  || ||/+-+++--+-++--+---++++-+-+-+++---+++++-+---+-++++--+----+-----+++++-+-+++++++--++---\|| ||| |   ||||||   |||| | | |
| |\+-+-++++-++---+++--+++-+--++-++++-+/| /+-++--+---++++-+-+-+++-\ ||||| |   | ||||  |    |     ||||| | |||||||  ||   ||| ||| |   ||||||   |||| | | |
| | | | |||| ||   |||  ||| |  || |||| | | || ||  |   |||| | | ||| | ||||| |/--+-++++--+----+-----+++++-+-+++++++--++-\ |||/+++-+---++++++---++++\| | |
| | | | |||| ||   |||  ||| |  |\-++++-+-+-++-++--+---++++-+-+-+++-+-+++/| ||  \-++++--+----+-----+++++-+-+++++++--++-+-++++/|| |   ||||||   |||||| | |
| | | | |||| ||   |||  ||| |  |  |||| | | || ||  |   |||| | | \++-+-+++-+-++----++++--+----+-----+++/| | |||||||  || | |||| || |   ||||||   |||||| | |
| | | | |||| ||   |||  ||| |  |/-++++-+-+-++-++--+---++++-+-+--++-+-+++-+\||    ||||  |    |     ||| \-+-+++++++--++-+-++++-++-+---++++++---+++++/ | |
| | | | ||||/++---+++-\||| |  || |||| | | || || /+---++++-+-+--++-+-+++-++++----++++--+----+-----+++-\ | |||||||  || | ||||/++-+---++++++---+++++\ | |
| | | | |v|||||   ||| |||| |  || |||| | | || || ||   |||| | |  || | ||| ||||    ||\+--+----+-----+++-+-+-+++++++--++-+-+++++++-+---/|||||   |||||| | |
| | | | |||||||   ||| |||| |  || |||| | | || || ||   |||| | |  || | ||| ||||    || |  |    |     ||| | \-+++++++--++-+-+++++++-+----+++++---++++++-+-/
| | | | |||||||   ||| |||| |  || |||| | |/++-++-++---++++-+-+--++-+-+++-++++----++-+--+----+-----+++-+---+++++++--++-+-+++++++-+----+++++---++++++\|  
| | | | |||||||   ||| |||| |  || |||| | |||| || ||   |||| | |/-++-+-+++-++++-\  || |  |    | /---+++-+---+++++++--++-+-+++++++\|    |||||   ||||||||  
| | | | |||||||   ||| |||| |  || |||| | |||| || ||   |||| | \+-++-+-+++-++++-+--++-+--+----+-+---+++-+---++/||||  || | |||||||||    |||||   ||||||||  
| | | | |||||||   ||| |||| |  || |||| | |||| ||/++---++++-+--+-++-+-+++-++++-+--++-+--+----+-+---+++-+---++-++++\ || | |||||||||    |||||   ||||||||  
| | | | |||||||   ||| |||| |  || |||| | |||| |||||   |||| |  | || | ||| |||| |  || |  |    | |   ||| |   || ||||| || | |||||||||    |||||   ||||||||  
| | | \-+++++++---+++-++++-+--++-++++-+-++++-+++++---/||| |  | || | ||| |||| |  || |  |   /+-+---+++-+---++-+++++-++-+-+++++++++----+++++---++++++++\ 
| | |   |||||||   ||| |||| |/-++-++++-+-++++-+++++----+++-+--+-++-+-+++-++++-+--++-+--+\  || |   ||| |   || ||||| || | |||||||||    |||||   ||||||||| 
| | |   ||||||| /-+++-++++-++-++-++++-+-++++-+++++----+++-+--+-++-+-+++-++++-+--++>+--++\ || |   ||| |   || ||||| || | |||||||||    |||||   ||||||||| 
| | |   |||||||/+-+++-++++\|| || |||| | |||| |||||   /+++-+--+-++-+-+++-++++-+--++-+--+++-++-+--\||| |   || |||||/++-+-+++++++++----+++++--\||||||||| 
| | |   ||||||||| ||| ||||||| || |||| | |||| |||||   |||| |/-+-++-+-+++-++++-+--++-+--+++-++-+--++++-+--\|| |||||||| | |||||||||    |||||  |||||||||| 
\-+-+---+++++++++-+++-+++++++-++-++++-+-++++-+++++---+++/ || | || | ||| |||| | /++-+--+++-++\|  |||| |  ||| |||||||| | |||||||||    |||||  |||||||||| 
  | |   ||||||||^ ||| ||||||| || |||| | |||| \++++---+++--++-+-++-+-/|| |||| | ||| |  ||| ||||  |||| |  ||| |||||||| | |||||||||    |||||  |||||||||| 
  | |   ||||||||| ||| ||||||| ^| |||| | ||||  ||||   |||  || | || |  || |||| | ||| |  ||| ||||  |||| |  ||| |||||||| | |||||||||    |||||  |||||||||| 
  | |   ||||||||| ||| ||||||| || |||| | ||||  ||||   |||  || | || |  || |||| |/+++-+--+++<++++--++++-+--+++-++++++++-+\|||||||||    |||||  |||||||||| 
  | |   ||||||||| ||| ||||||| || |||| | ||||  ||||   |||  || | || |  || |||| ||||| |  ||| ||||  |||| |  ||| |||||||| |||||||||||    |||||  |||||||||| 
  | |   ||||||||| ||\-+++++++-++-++++-+-++++--++++---+++--++-+-++-+--++-++++-+++++-+--+++-++++--++++-+--+++-++++++++-+++++++/|||    |||||  |||||||||| 
  | |   ||||||||| ||  ||||||| || |\++-+-++++--++++---+++--/| | || |  || |||| ||||| |  ||| ||||  |||| |  ||| |||||||| ||||||| |||    |||||  |||||||||| 
  | |   ||||||||| ||  ||||||| || | || | ||||  ||||   |||   | | |\-+--++-++++-+++++-+--+++-++++--++++-+--+/| |||||||| ||||||| |||    |||||  |||||||||| 
  | |   ||||||||| ||  ||||||| || | || | ||||  ||||   |||   | | |  |  || |||| ||||| |  ||| ||||  |||| |  | | |||||||| ||||||| |||    |||||  |||||||||| 
  | |   ||||||||| ||  ||||||| || \-++-+-++++--++++---+++---+-+-+--+--++-++++-++++/ |  ||| ||||  |||| |  | | |||||||| ||||||| |||    |||||  |||||||||| 
  | |   ||||||||| ||  ||||||| ||   || \-++++--++++---+++---+-+-+--+--++-++++-++++--+--+++-++++--++++-+--+-+-++++++++-+++++++-+++----+/|||  |||||||||| 
  | |   ||||||||| ||  ||||||| ||   ||   ||||  ||||   |||   \-+-+--+--++-++++-++++--+--+++-++++--++++-+--/ | |||||||| ||||||| |||    | |||  |||||||||| 
  | |   ||||\++++-++--/|||||| ||   ||   ||||  ||||   |||     | |  |  || |||| ||||  |  ||| \+++--++++-+----+-++++++++-+++++++-+++----+-+++--+++++++++/ 
  \-+---+++/ |||| ||   |||||| ||   ||   ||||  ||||   |||     | |  |  || |||| ||||  |  |||  |||  |||| |    | |||||||| ||||||| |||    | |||  |||||||||  
    |   |||  |||| ||   |||||| ||   ||   ||||  ||||   |||     | |  |  || |||| ||||  |  |||  |||  |||| |    | |||||||| ||||||| |||    | |||  |||||||||  
    |   |||  |||| ||   |||||| || /-++---++++-\||\+---+++-----+-+--+--++-++++-++++--+--+++--+++--++++-/    | |||||||| |||\+++-+++----+-+++--++++/||||  
    |   |||  |||| ||   |||||| || | ||   |||| ||| |   |||     | |  |  \+-++++-++++--+--+++--+++--++++------+-++++++++-+++-+++-+++----+-+++--+++/ ||||  
    |   |||  |||| |^   |||||| || | ||   |||| ||| |   |||     | |  |   | |||| ||||  |  |||  |||  |||| /----+-++++++++-+++-+++-+++----+-+++-\|||  ||||  
    |   |||  |||| ||   ||||\+-++-+-++---++++-+/| |   |||     | |  |   | |||| ||||  |  |||  |||  |||| |    | |||||||\-+++-+++-+++----+-+/| ||||  ||||  
    |   |||  |||| \+---++++-+-++-+-++---++++-+-+-+---+++<----+-+--+---+-++++-++++--+--+++--+++--+++/ |    | |||||||  ||| ||| |||    | | | ||||  ||||  
    |   |||  ||||  |   |||| | || | ||   |||| | | |   \++-----+-+--+---+-++++-++++--+--+++--+++--/|\--+----+-+++++++--+++-+++-+++----+-+-+-++++--+++/  
    |   |||  ||||  |   |||| | || | ||   |||| | | |    ||   /-+-+--+---+-++++-++++--+--+++--+++---+---+----+-+++++++--+++-+++\|||    | | | ||||  |||   
    |   |||  ||||  |   |||| | || | ||   ||\+-+-+-+----++---+-+-+--/   | |||| ||||  |  |||  |||   |   |    | |||||||  ||| |||||||    | | | ||||  |||   
    |   |||  ||||  |   |||| | || | ||   || | | | |    ||   | | |      | |||| ||||  |/-+++--+++---+---+----+-+++++++-\||| |||||||    | | | ||||  |||   
    |   |||/-++++--+---++++-+-++-+-++---++-+-+-+-+----++---+-+-+---\  | |||| ||||  || |||  |||   |   |    | |||||||/++++-+++++++----+-+-+-++++-\|||   
    |   |||| ||||  |   |||| | || | ||   || |/+-+-+----++---+-+-+---+--+-++++-++++--++-+++--+++---+---+----+-++++++++++++-+++++++----+-+-+-++++-++++-\ 
    |   |||| ||||  |   |||| | || | ||   || ||| | |  /-++---+-+\|   |  | |||| ||||  || |||  |||   |   |    | |||||||||||| |||||||    | | | |||| |||| v 
    |   |||| ||||  |   |||| | || | ||   || ||| | |  | ||   | |||   |  | |||| ||||  || |||  |||   |   |    | |||||||||||| |||||||    | | | |||| |||| | 
    |   |||| \+++--+---++++-+-++-+-++---++-+++-+-+--+-++---+-+++---+--+-++++-++++--++-+++--+++---+---+----+-+/|||||||||| |||||||    | | | |||| |||| | 
    |   ||||  |||  |   |||| | || | ||/--++-+++-+-+--+-++---+-+++---+--+-++++-++++--++-+++--+++---+---+\   | | |||||||||| |||||||    | | | |||| |||| | 
    |   ||||  |||  |   |||| | || | |||  || ||| | |  |/++---+-+++---+--+-++++-++++--++-+++--+++---+---++---+-+-++++++++++-+++++++----+-+\| |||| |||| | 
    |   ||||  |||  \---++++-+-++-+-+++--++-+++-+-+--++++---+-+++---+--+-++++-++++--/| |||  |||   |   ||   | | |||||||||| |||||||    | ||| |||| |||| | 
 /--+---++++--+++------++++-+-++-+-+++--++-+++-+-+\ ||||   | |||   |  | |||| ||||   | |||  |||   |   ||   | | |||||||||| |||||||    | ||| |||| |||| | 
 |  \---++++--+++------++++-+-++-+-+++--++-+++-+-++-++++---+-+++---+--+-++++-+++/   | |||  |||   |   ||   | | |||||||||| |||||||    | ||| |||| |||| | 
 |      ||||  |||      |\++-+-++-+-+++--++-+++-+-++-++++---+-+++---+--/ |||| |||    \-+++--+++---+---++---+-+-++++++/||| |||||||    | ||| |||| |||| | 
 |      |||\--+++------+-++-+-++-+-+++--++-+++-+-++-++++---+-+++---/   /++++-+++------+++--+++---+---++---+-+-++++++-+++-+++++++----+-+++-++++-++++-+\
 |      ||\---+++------+-/| | || | |||  || ||| | || ||||   | |||       ||||| |||      |||  |||   |   \+---+-+-++++++-+++-+++++++----+-+++-/||| |||| ||
 |      ||    |||      |  | | || | |||  || ||| | || ||||   | |||       ||||| |||      |||  |||   |    |   | | |||||| ||| |||||||    | |||  ||| |||| ||
 |      ||    |||      |  | | || | |||  || ||| | || ||||/--+-+++-------+++++-+++------+++--+++\  |    |   | | |||||| ||| |||||||    | |||  ||| |||| ||
/+------++----+++\     |  | | || | |||  || |\+-+-++-+++++--+-+++-------+++++-+++------+++--++++--+----+---+-+-++++++-+++-+++++++----+-+++--+++-++++-/|
||      ||    |\++-----+--/ | || | ||| /++-+-+-+-++-+++++--+-+++-------+++++-+++------+++--++++--+----+---+-+\|||||| ||| |||||||    | |||  ||| ||||  |
||      |\----+-++-----+----+-++-+-+++-+++-+-+-+-++-+++++--+-+++-------+++++-+++------+++--++++--+----+---+-++++++/| ||| |||||||    | |||  ||| ||||  |
||      |     | ||     |    | \+-+-+++-+++-+-+-+-++-+++++--+-+++-------+++++-+++------+++--++++--+----+---+-++++++-+-+++-+++++++----+-/||  ||| ||||  |
||      |     \-++-----+----+--+-+-+++>+++-+-+-+-++-+++++--+-++/       ||||| |||      |||  ||||  |    |   | ||||||/+-+++-+++++++----+--++--+++\||||  |
||/-----+-------++-----+----+--+-+-+++\||| | | | || |||||  \-++--------+++++-+++------+++--++++--+----+---+-++++++++-+++-+++/|||    |  ||  ||||||||  |
|||     |       ||     |    |  | | |||||\+-+-+-+-++-+++++----++--------+++++-+++------+++--++++--+----+---+-++/||||| ||| ||| |||    |  ||  ||||||||  |
|||     |       ||     |    |  | | ||||| | | | | || ||\++----++--------+++++-+++------+++--++++--+----+---+-++-+++++-+++-+++-+++----/  ||  ||||||||  |
|||   /-+-------++-----+-\  |  | | |\+++-+-+-+-+-++-++-++----++--------+++++-+++------/||  ||||  |    |   | || ||||| ||| ||| \++-------++--++/|||||  |
|||   | |       ||     | |  |  | | | ||| | | | | || || ||    ||        ||||| |||       || /++++--+----+---+-++-+++++-+++-+++--++-------++--++-+++++\ |
|||   | |       ||     | |  |  | | | ||| | | | | || || ||    ||/-------+++++-+++-------++-+++++--+----+--\| || ||||| ||| |||  ||       ||  || |||||| |
|||   | \-->----++-----+-+--+--+-+-+-+++-+-+-+-+-/| || ||    |||       ||||| ||\-------++-++/||  |    |  || || ||||| ||| |||  ||       ||  || |||||| |
\++---+---------+/     | |  |  | | | ||| | \-+-+--+-++-++----+++-------+++++-++--------++-++-++--+----+--++-++-/|||| ||| |||  ||       ||  || |||||| |
 ||   |         |      | |  |  | | | ||| |   | |  | || ||    |||       ||||| |\--------++-++-++--+----+--++-++--++++-+/| |||  ||       ||  || |||||| |
 ||   |         |      | |  |  |/+-+-+++-+---+-+--+-++-++----+++-------+++++-+--\      || || ||  |    |  || ||  |||| | | |\+--++-------++--++-++/||| |
 ||   |         |      \-+--+--+++-+-+++-+---+-+--+-++-++----+++-------+++++-+--+------++-++-++--/    |  || ||  |||| | | | \--++-------++--++-++-/|| |
 ||   |         |        |  |/-+++-+-+++-+---+-+--+\|| ||    |||       ||||| |  |      || |\-++-------+--++-++--++++-+-+-/    ||       ||  || ||  || |
 ||   |         |        |  || ||| | ||| |   | \--++++-++----+++-------+++++-+--+------++-+--++-------+--++-++--/\++-+-+------++-------++--/| ||  || |
 ||   |         |        |  || ||| | \++-+---+----++++-++----+++-------+++++-+--+------++-+--++-------/  || ||    || | |      ||       ||   | ||  || |
 ||   |   /-----+--------+--++-+++-+--++-+---+-\  |||| ||    |||       |\+++-+--+------++-+--++----------+/ ||    || | |      ||       ||   | ||  || |
 ||   \---+-----+--------/  || ||| |  || |   | |  ||\+-++----+/|       | ||| |  |      || |  ||          |  ||    || | |      ||       ||   | ||  || |
 ||    /--+-----+-----------++-+++-+--++-+\  | |  || | ||    | |       | ||| |  |      || |  ||          |  ||    || | |      ||       ||   | ||  || |
 ||    |  |     |           || ||| |  || \+--+-+--++-+-++----+-+-------+-+++-+--+------++-+--++----------+--++----++-+-+------++-------++---+-++--/| |
 ||    |  |     |     /-----++-+++-+--++--+--+-+--++-+-++-\  | |       | ||| |  |      || |  ||          |  ||    || | |      ||       ||   | ||   | |
 ||    |  |     |     |     || ||| |  ||  |  | |  || | || |  | |       | ||| |  |      || \--++----------+--++----++-+-+------++-------++---+-++---/ |
 ||    |  |     |     |     || ||| |  ||  |  | |  || | |\-+--+-+-------+-+++-+--+------++----+/          |  \+----++-+-+------+/       ||   | ||     |
 \+----+--+-----+-----+-----++-+++-+--++--+--+-+--/| | |  |  | |       | ||| |  |      ||    |           |   |    || | |      |      /-++---+-++-\   |
  |    |  |     |     |     || ||| |  ||  |  | |   | | |  |  | |       | ||| |  |      ||    |           |   |    |v | |      |      | ||   | || |   |
  |    |  |     |     |     || |\+-+--++--+--+-+---+-+-+--+--+-+<------+-+++-+--/      ||    |           |   |    || | |      |      | ||   | || |   |
  |    |  |     |     |     || \-+-+--++--+--+-+---+-+-+--+--+-+-------+-/|| |         ||    |           |   |    || | |      |      | ||   | || |   |
  |    |  |     \-----+-----++---+-+--++--+--+-+-<-+-+-+--+--+-+-------+--++-+---------+/    |           |   |    || | |      |      | ||   | || |   |
  |    |  |           |     ||   | |  ||  |  | |   | | \--+--+-+-------+--++-+---------+-----+-----------+---+----++-+-+------+------+-+/   | || |   |
  |    |  |         /-+-----++---+-+--++--+--+-+---+-+----+--+-+-------+--++-+------\  |     |           |   |    || | |      |      | |    | || |   |
  |    |/-+---------+-+-----++\  | |  |\--+--+-+---+-+----+--+-+-------+--++-+------+--+-----+-----------+-->/    \+-+-+------+------+-+----+-/| |   |
  |    || \---------+-+-----+++--+-+--+---+--+-/   | |    |  | |       |  || |      |  |     |           |         | | |      |      | |    |  | |   |
  |    ||           | |     |||  | |  |   |  |     | |    |  | |       |  || |      |  |     |           |         \-+-+------+------+-+----+--/ |   |
  \----++-----------+-+-----+++--+-+--/   |  |     | |    |  \-+-------+--++-/      |  |     |           |           | |      |      | |    |    |   |
       ||           | |     |\+--+-+------+--+-----/ |    |    |       |  |\--------+--+-----+-----------+-----------/ |      |      | |    |    |   |
       ||           | |     | |  | |      |  |       \----+----+-------+--+---------+--+-----+-----------+-------------+------+------+-/    |    |   |
       \+-----------+-+-----+-+--+-+------/  |            |    |       |  |         |  |     \-----------+-------------+------/      |      |    |   |
        |           | |     \-+--+-+---------+------------+----+----->-+--+---------+--/                 |             |             \------+----/   |
        |           | |       |  \-+---------/            |    \-------+--+---------+--------------------/             |                    |        |
        \-----------+-+-------/    \----------------------+------------+--+---------+----------------------------------/                    |        |
                    | \-----------------------------------/            |  \---------+-------------------------------------------------------/        |
                    \--------------------------------------------------+------------/                                                                |
                                                                       \-----------------------------------------------------------------------------/
"#;
