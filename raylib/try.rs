
/*
 typedef struct Color {
    unsigned char r;        // Color red value
    unsigned char g;        // Color green value
    unsigned char b;        // Color blue value
    unsigned char a;        // Color alpha value
} Color;
*/

//https://doc.rust-lang.org/nomicon/other-reprs.html
#[repr(C)]
struct Color{
   r:std::ffi::c_uchar,
   g:std::ffi::c_uchar,
   b:std::ffi::c_uchar,
   a:std::ffi::c_uchar
}

impl Clone for Color{
   fn clone(&self)->Color{
      Color{r:self.r,g:self.g,b:self.b,a:self.a}
   }
}

#[repr(C)]
struct Circle{
   posx:std::ffi::c_int,
   posy:std::ffi::c_int,
   radius:std::ffi::c_float,
   color:Color,
   speedx:std::ffi::c_int,
   speedy:std::ffi::c_int
}

impl Circle{
   fn default_new()->Self{
      Self{
         posx:300,
         posy:300,
         radius:22.5,
         color:Color{r:255,g:122,b:40,a:255},
         speedx:12,
         speedy:5
      }
   }
   fn moveX(&mut self){
      self.posx+=self.speedx;
   }
   fn boundX(&mut self,bound:std::ffi::c_int){
      if self.posx<0 || self.posx>bound{
         self.speedx*=-1;
      }
   }
   fn moveY(&mut self){
      self.posy+=self.speedy;
   }
   fn boundY(&mut self,bound:std::ffi::c_int){
      if self.posy<0 || self.posy>bound{
         self.speedy*=-1;
      }
   }
}

//see https://doc.rust-lang.org/nomicon/ffi.html
#[link(name="m")]
#[link(name="X11")]
#[link(name="raylib",kind="static")]
unsafe extern "C"{
  fn InitWindow(width:std::ffi::c_int,height:std::ffi::c_int,title:*const std::ffi::c_char);
  fn CloseWindow();
  //https://stackoverflow.com/questions/47705093/what-is-the-correct-type-for-returning-a-c99-bool-to-rust-via-the-ffi
  //2018 rust bool is same as c bool
  fn WindowShouldClose()->bool;
  fn BeginDrawing();
  fn EndDrawing();
  //void ClearBackground(Color color); 
  fn ClearBackground(color:Color);
  //void DrawCircle(int centerX, int centerY, float radius, Color color);
  fn DrawCircle(centerX:std::ffi::c_int,centerY:std::ffi::c_int,radius:std::ffi::c_float,color:Color);
}

const WIDTH:i32=600;
const HEIGHT:i32=600;

fn main(){
   let mut cir:Circle=Circle::default_new();
   unsafe{
   InitWindow(WIDTH,HEIGHT,std::ffi::CString::new("hello").unwrap().as_ptr() as *const std::ffi::c_char);
   while WindowShouldClose()==false{
        BeginDrawing();
        ClearBackground(Color{r:40,g:40,b:40,a:255});
        DrawCircle(cir.posx,cir.posy,cir.radius,cir.color.clone());
        cir.moveX();
        cir.boundX(WIDTH);
        cir.moveY();
        cir.boundY(HEIGHT);
        std::thread::sleep(std::time::Duration::from_millis(1000)/100);
        EndDrawing();
      }
      CloseWindow();
   }
}
